import { GetStaticPropsResult } from "next";
import Link from "next/link";
import path from "path";
import MainLayout from "layouts/main";
import { postsPath, postSlugs } from "constants/posts";
import { bundleMDX } from "mdx-bundler";
import BlogCard from "@/blog-card";

interface Props {
  posts: Post[];
}

export default function Index({ posts }: Props) {
  return (
    <MainLayout>
      <h1 className="text-8xl font-black">{"posts"}</h1>
      <div className="flex flex-col gap-4 justify-center items-center w-full">
        {posts.map((post) => (
          <BlogCard
            slug={post.slug.replace(/\.mdx?$/, "")}
            key={post.slug}
            {...post.frontmatter}
          />
        ))}
      </div>
    </MainLayout>
  );
}

async function getStaticProps(): Promise<GetStaticPropsResult<Props>> {
  const postsPromise = postSlugs.map(async (slug): Promise<Post> => {
    const { frontmatter, code } = await bundleMDX<FrontmatterProps>({
      file: path.join(postsPath, slug),
    });

    const imageURL = frontmatter.imageURL ?? null;
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
        imageURL,
        dateAdded,
        dateEdited,
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
