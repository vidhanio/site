import fs from "fs";
import matter from "gray-matter";
import { GetStaticPropsResult } from "next";
import Link from "next/link";
import path from "path";
import { H1 } from "@/elements/headings";
import MainLayout from "layouts/main";
import { postsPath, postFilePaths } from "utils/mdx";

interface Props {
  posts: {
    content: string;
    frontmatter: {
      [key: string]: any;
    };
    path: string;
  }[];
}

export default function Index({ posts }: Props) {
  return (
    <MainLayout>
      <H1>{"vidhan's blog"}</H1>
      <ul>
        {posts.map((post) => (
          <li key={post.path}>
            <Link
              as={`/posts/${post.path.replace(/\.mdx?$/, "")}`}
              href={`/posts/[slug]`}
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
  const posts = postFilePaths.map((filePath) => {
    const source = fs.readFileSync(path.join(postsPath, filePath));
    const { content, data } = matter(source);

    return {
      content,
      frontmatter: data,
      path: filePath,
    };
  });

  return {
    props: {
      posts: posts,
    },
  };
}
