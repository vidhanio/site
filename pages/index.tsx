import fs from "fs";
import matter from "gray-matter";
import { GetStaticPropsResult } from "next";
import Link from "next/link";
import path from "path";
import { H1 } from "@/elements";
import MainLayout from "layouts/main";
import { postsPath, postSlugs } from "constants/posts";
import { bundleMDX } from "mdx-bundler";

interface FrontmatterProps {
  title: string;
  description: string;
  dateAdded: Date;
  dateEdited?: Date | undefined;
}

interface Post {
  slug: string;
  code: string;
  frontmatter: {
    title: string;
    description: string;
    dateAdded: [number, number, number];
    dateEdited?: [number, number, number] | null;
  };
}

interface Props {
  posts: Post[];
}

export default function Index({ posts }: Props) {
  return (
    <MainLayout>
      <H1>{"vidhan's blog"}</H1>
      <ul>
        {posts.map((post) => (
          <li key={post.slug}>
            <Link href={`/post/${post.slug.replace(/\.mdx?$/, "")}`}>
              <a>{post.frontmatter.title}</a>
            </Link>
          </li>
        ))}
      </ul>
    </MainLayout>
  );
}

async function getStaticProps(): Promise<GetStaticPropsResult<Props>> {
  const postsPromise = postSlugs.map(async (slug): Promise<Post> => {
    const { frontmatter, code } = await bundleMDX<FrontmatterProps>({
      file: path.join(postsPath, slug),
    });

    const dateAdded = [
      frontmatter.dateAdded.getUTCFullYear(),
      frontmatter.dateAdded.getUTCMonth(),
      frontmatter.dateAdded.getUTCDate(),
    ] as [number, number, number];
    const dateEdited = frontmatter.dateEdited
      ? ([
          frontmatter.dateEdited.getUTCFullYear(),
          frontmatter.dateEdited.getUTCMonth(),
          frontmatter.dateEdited.getUTCDate(),
        ] as [number, number, number])
      : null;

    return {
      slug,
      code,
      frontmatter: {
        ...frontmatter,
        dateAdded: dateAdded,
        dateEdited: dateEdited,
      },
    };
  });

  return {
    props: {
      posts: await Promise.all(postsPromise),
    },
  };
}

export { getStaticProps };
