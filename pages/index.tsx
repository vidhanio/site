import fs from "fs";
import matter from "gray-matter";
import { GetStaticPropsResult } from "next";
import Link from "next/link";
import path from "path";
import { H1 } from "@/elements/headings";
import MainLayout from "layouts/main";
import { postPath, postFilePaths } from "utils/mdx";

interface Props {
  post: {
    content: string;
    frontmatter: {
      [key: string]: any;
    };
    path: string;
  }[];
}

export default function Index({ post }: Props) {
  return (
    <MainLayout>
      <H1>{"vidhan's blog"}</H1>
      <ul>
        {post.map((post) => (
          <li key={post.path}>
            <Link
              as={`/post/${post.path.replace(/\.mdx?$/, "")}`}
              href={`/post/[slug]`}
            >
              <a>{post.frontmatter.title}</a>
            </Link>
          </li>
        ))}
      </ul>
    </MainLayout>
  );
}

export async function getStaticProps(): Promise<GetStaticPropsResult<Props>> {
  const post = postFilePaths.map((filePath) => {
    const source = fs.readFileSync(path.join(postPath, filePath));
    const { content, data } = matter(source);

    return {
      content,
      frontmatter: data,
      path: filePath,
    };
  });

  return {
    props: {
      post: post,
    },
  };
}
