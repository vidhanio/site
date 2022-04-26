import { FrontmatterProps, Post } from "types";

import { bundleMDX } from "mdx-bundler";
import path from "path";
import { postsPath } from "constants/posts";
import rehypePrism from "rehype-prism-plus";
import simpleGit from "simple-git";

export default async function postFromSlug(slug: string): Promise<Post> {
  const filePath = path.join(postsPath, `${slug}.mdx`);

  const { frontmatter, code: content } = await bundleMDX<FrontmatterProps>({
    file: filePath,
    mdxOptions(options) {
      options.rehypePlugins = [[rehypePrism, { showLineNumbers: true }]];
      return options;
    },
  });

  const git = simpleGit();

  const commits = (
    await git.log({
      file: filePath,
    })
  ).all;

  const firstCommit = commits[commits.length - 1];
  const lastCommit = commits[0];

  const [dateAdded, hashAdded] = firstCommit
    ? [firstCommit.date, firstCommit.hash]
    : [new Date().toISOString(), ""];

  const [dateUpdated, hashUpdated] = firstCommit
    ? firstCommit.hash !== lastCommit.hash
      ? [lastCommit.date, lastCommit.hash]
      : [null, null]
    : [null, null];

  return {
    title: frontmatter.title,
    description: frontmatter.description,
    imageURL: frontmatter.imageURL ?? null,
    imageAlt: frontmatter.imageAlt ?? null,
    slug,
    content,
    dateAdded,
    dateUpdated,
    hashAdded,
    hashUpdated,
  };
}
