use std::{
    collections::{HashMap, hash_map::Entry},
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};

use time::{OffsetDateTime, UtcOffset};
use typst::{
    Library, LibraryExt, World,
    diag::{FileError, FileResult, SourceDiagnostic},
    ecow::EcoVec,
    foundations::{Bytes, Datetime, Duration, IntoValue, Str},
    syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot},
    text::{Font, FontBook},
    utils::LazyHash,
};
use typst_kit::fonts::FontStore;
use typst_layout::PagedDocument;

use crate::Error;

pub struct SiteWorld {
    library: LazyHash<Library>,
    main: Source,
    fs: FileSystem,
    fonts: FontStore,
    now: OffsetDateTime,
}

impl SiteWorld {
    pub fn new(
        project_root: impl AsRef<Path>,
        main_path: impl AsRef<Path>,
        inputs: impl IntoIterator<Item = (impl Into<Str>, impl IntoValue)>,
    ) -> Result<Self, Error> {
        let project_root = project_root.as_ref().to_owned();
        let main_path = main_path.as_ref();
        let virtual_path = VirtualPath::virtualize(&project_root, main_path)
            .map_err(|_| Error::InvalidPath(main_path.to_owned()))?;
        let main_file_id = FileId::new(RootedPath::new(VirtualRoot::Project, virtual_path));
        let main_file_contents = fs::read_to_string(main_path)?;

        let mut fonts = FontStore::new();
        fonts.extend(typst_kit::fonts::scan(
            &project_root.join("assets/static/fonts"),
        ));

        Ok(Self {
            fs: FileSystem::new(project_root),
            library: LazyHash::new(
                Library::builder()
                    .with_inputs(
                        inputs
                            .into_iter()
                            .map(|(key, value)| (key.into(), value.into_value()))
                            .collect(),
                    )
                    .build(),
            ),
            main: Source::new(main_file_id, main_file_contents),
            fonts,
            now: OffsetDateTime::now_utc(),
        })
    }

    pub fn compile_document(&self) -> Result<PagedDocument, Error> {
        let warned = typst::compile::<PagedDocument>(self);

        if !warned.warnings.is_empty() {
            return Err(diagnostic_error(warned.warnings));
        }

        warned.output.map_err(diagnostic_error)
    }
}

impl World for SiteWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        self.fonts.book()
    }

    fn main(&self) -> FileId {
        self.main.id()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main.id() {
            Ok(Source::clone(&self.main))
        } else {
            self.fs.source(id)
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.fs.file(id)
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.font(index)
    }

    fn today(&self, offset: Option<Duration>) -> Option<Datetime> {
        let offset = match offset {
            None => UtcOffset::UTC,
            Some(offset) => {
                let seconds = i32::try_from(time::Duration::from(offset).whole_seconds()).ok()?;
                UtcOffset::from_whole_seconds(seconds).ok()?
            }
        };
        let datetime = self.now.checked_to_offset(offset)?;
        Some(Datetime::Date(datetime.date()))
    }
}

struct FileSystem {
    project_root: PathBuf,
    files: Mutex<HashMap<FileId, FileEntry>>,
}

impl FileSystem {
    fn new(project_root: PathBuf) -> Self {
        Self {
            project_root,
            files: Mutex::new(HashMap::new()),
        }
    }

    fn map_file<T: Clone>(
        &self,
        id: FileId,
        f: impl FnOnce(&mut FileEntry) -> FileResult<T>,
    ) -> FileResult<T> {
        let mut files = self.files.lock().unwrap();

        match files.entry(id) {
            Entry::Occupied(entry) => Ok(f(entry.into_mut())?),
            Entry::Vacant(entry) => {
                let path = id
                    .vpath()
                    .realize(&self.project_root)
                    .map_err(FileError::Realize)?;
                let bytes = fs::read(&path).map_err(|error| FileError::from_io(error, &path))?;
                let mut file_entry = FileEntry::new(bytes);
                let result = f(&mut file_entry)?;
                entry.insert(file_entry);
                Ok(result)
            }
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.map_file(id, |file_entry| Ok(Bytes::clone(&file_entry.bytes)))
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        self.map_file(id, |file_entry| file_entry.source(id))
    }
}

#[derive(Clone)]
struct FileEntry {
    bytes: Bytes,
    source: Option<Source>,
}

impl FileEntry {
    fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes: Bytes::new(bytes),
            source: None,
        }
    }

    fn source(&mut self, id: FileId) -> FileResult<Source> {
        let source = if let Some(source) = &self.source {
            Source::clone(source)
        } else {
            let contents = str::from_utf8(&self.bytes).map_err(|_| FileError::InvalidUtf8)?;
            let source = Source::new(id, contents.into());
            Source::clone(self.source.insert(source))
        };

        Ok(source)
    }
}

#[expect(clippy::needless_pass_by_value)]
pub fn diagnostic_error(diagnostics: EcoVec<SourceDiagnostic>) -> Error {
    Error::Typst {
        diagnostics: diagnostics
            .iter()
            .map(|diagnostic| format!("{diagnostic:?}"))
            .collect::<Vec<_>>()
            .join("\n"),
    }
}
