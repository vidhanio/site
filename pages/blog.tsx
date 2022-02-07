import { GetStaticPropsResult, InferGetStaticPropsType } from "next";
import Posts, { PostCard } from "components/blog/card";

import H1 from "components/elements/h1";
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
      <H1>blog</H1>
      <Posts posts={posts}></Posts>
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
