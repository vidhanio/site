import fs from "fs";
import path from "path";

const postsPath = path.join(process.cwd(), "posts");

const postFilePaths = fs
  .readdirSync(postsPath)
  .filter((path) => /\.mdx?$/.test(path));

export { postsPath, postFilePaths };
