import { bundleMDX } from "mdx-bundler";
import path from "path";
import { postsPath } from "constants/posts";
import rehypePrism from "rehype-prism-plus";
import simpleGit from "simple-git";

async function PostFromSlug(slug: string): Promise<Post> {
  const filePath = path.join(postsPath, `${slug}.mdx`);

  const { frontmatter, code: content } = await bundleMDX<FrontmatterProps>({
    file: filePath,
    xdmOptions(options) {
      options.rehypePlugins = [[rehypePrism, { showLineNumbers: true }]];
      return options;
    },
  });

  const git = simpleGit();

  const commits = await git.log({
    file: filePath,
  });

  const firstCommit = commits.all[commits.all.length - 1];
  const lastCommit = commits.all[0];

  const dateAdded = firstCommit.date;
  const dateUpdated =
    firstCommit.hash !== lastCommit.hash ? lastCommit.date : null;

  const imageURL = frontmatter.imageURL ?? null;

  return {
    title: frontmatter.title,
    description: frontmatter.description,
    imageURL,
    slug,
    content,
    dateAdded,
    dateUpdated,
  };
}

export default PostFromSlug;
