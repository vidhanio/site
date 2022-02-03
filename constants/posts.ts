import fs from "fs";
import path from "path";

const postsPath = path.join(process.cwd(), "content", "posts");

const postSlugs = fs
  .readdirSync(postsPath)
  .filter((file) => /\.mdx$/.test(file))
  .map((file) => file.replace(/\.mdx$/, ""));

export { postsPath, postSlugs };
