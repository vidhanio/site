use std::{
    collections::{HashMap, hash_map::Entry},
    error::Error,
    fs,
    path::Path,
    sync::{LazyLock, Mutex},
};

use time::{OffsetDateTime, UtcOffset};
use typst::{
    Library, World,
    diag::{FileError, FileResult, SourceDiagnostic},
    ecow::{EcoVec, eco_format},
    foundations::{Bytes, Datetime, Dict},
    layout::PagedDocument,
    syntax::{FileId, Source, VirtualPath},
    text::{Font, FontBook},
    utils::LazyHash,
};
use typst_kit::fonts::{FontSearcher, FontSlot};

use crate::{CARGO_MANIFEST_DIR, FONTS_DIR};

static FONTS: LazyLock<(LazyHash<FontBook>, Vec<FontSlot>)> = LazyLock::new(|| {
    let fonts = FontSearcher::new()
        .include_system_fonts(false)
        .search_with([&*FONTS_DIR]);

    (LazyHash::new(fonts.book), fonts.fonts)
});

#[derive(Debug)]
pub struct SiteWorld {
    library: LazyHash<Library>,
    main: Source,
    fs: FileSystem,
    now: OffsetDateTime,
}

impl SiteWorld {
    pub fn new(main_path: impl AsRef<Path>, inputs: Dict) -> FileResult<Self> {
        let main_path = main_path.as_ref();
        let main_file_id = FileId::new(
            None,
            VirtualPath::within_root(main_path, &CARGO_MANIFEST_DIR).ok_or_else(|| {
                FileError::Other(Some(eco_format!(
                    "main file path `{}` is not within the project root `{}`",
                    main_path.display(),
                    CARGO_MANIFEST_DIR.display()
                )))
            })?,
        );
        let main_file_contents =
            fs::read_to_string(main_path).map_err(|e| FileError::from_io(e, main_path))?;

        Ok(Self {
            library: LazyHash::new(Library::builder().with_inputs(inputs).build()),
            main: Source::new(main_file_id, main_file_contents),
            fs: FileSystem::new(),
            now: OffsetDateTime::now_utc(),
        })
    }

    pub fn compile_document(&self) -> Result<PagedDocument, Box<dyn Error>> {
        let warned = typst::compile::<PagedDocument>(&self);

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
        &FONTS.0
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
        FONTS.1.get(index)?.get()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let offset = UtcOffset::from_hms(offset.unwrap_or(0).try_into().ok()?, 0, 0).ok()?;
        let datetime = self.now.checked_to_offset(offset)?;
        Some(Datetime::Date(datetime.date()))
    }
}

#[derive(Debug)]
struct FileSystem {
    files: Mutex<HashMap<FileId, FileEntry>>,
}

impl FileSystem {
    fn new() -> Self {
        Self {
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
                    .resolve(&CARGO_MANIFEST_DIR)
                    .ok_or_else(|| FileError::NotFound(id.vpath().as_rootless_path().into()))?;

                let bytes = fs::read(&path).map_err(|e| FileError::from_io(e, &path))?;

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

#[derive(Clone, Debug)]
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
pub fn diagnostic_error(diagnostics: EcoVec<SourceDiagnostic>) -> Box<dyn Error> {
    diagnostics
        .iter()
        .map(|d| format!("{d:?}"))
        .collect::<Vec<_>>()
        .join("\n")
        .into()
}
