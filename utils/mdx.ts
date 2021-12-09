import fs from "fs";
import path from "path";

const postPath = path.join(process.cwd(), "post");

const postFilePaths = fs
  .readdirSync(postPath)
  .filter((path) => /\.mdx?$/.test(path));

export { postPath, postFilePaths };
