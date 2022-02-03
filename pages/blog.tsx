import { GetStaticPropsResult, InferGetStaticPropsType } from "next";

import Card from "components/blog/card";
import { Post } from "types";
import PostFromSlug from "utils/post-from-slug";
import SEO from "components/seo";
import { postSlugs } from "constants/posts";

type Props = {
  posts: Post[];
};

export default function Index({
  posts,
}: InferGetStaticPropsType<typeof getStaticProps>) {
  return (
    <>
      <SEO path="blog" />
      <h1 className="text-8xl font-extrabold italic text-indigo-500">blog</h1>
      <div className="flex w-full flex-col items-center justify-center gap-4">
        {posts
          .sort(
            (a, b) =>
              new Date(b.dateAdded).getTime() - new Date(a.dateAdded).getTime()
          )
          .map((post) => (
            <Card key={post.slug} {...post} />
          ))}
      </div>
    </>
  );
}

export async function getStaticProps(): Promise<GetStaticPropsResult<Props>> {
  return {
    props: {
      posts: await Promise.all(
        postSlugs.map(async (slug): Promise<Post> => {
          return PostFromSlug(slug);
        })
      ),
    },
  };
}
