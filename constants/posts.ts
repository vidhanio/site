import fs from "fs";
import path from "path";

const postsPath = path.join(process.cwd(), "data", "posts");

const postSlugs = fs
  .readdirSync(postsPath)
  .filter((path) => /\.mdx?$/.test(path));

export { postsPath, postSlugs };
