import { GetStaticPropsResult, InferGetStaticPropsType } from "next";
import { MainLayout, SectionLayout } from "layouts/main";

import BlogPostCard from "components/blog-post-card";
import PostFromSlug from "utils/post-from-slug";
import Typewriter from "components/typewriter";
import { postSlugs } from "constants/posts";

type Props = {
  posts: Post[];
};

function Index({ posts }: InferGetStaticPropsType<typeof getStaticProps>) {
  return (
    <>
      <header className="flex flex-col gap-2 justify-center items-center w-full">
        <h1 className="text-8xl font-black text-indigo-500">
          {"vidhan's blog"}
        </h1>
        <p className="text-2xl font-semibold text-indigo-700 dark:text-indigo-300">
          <Typewriter
            className="font-bold text-emerald-600 dark:text-emerald-400"
            prefix="read my "
            strings={["thoughts", "ramblings", "ideas", "opinions"]}
            suffix="."
          />
        </p>
      </header>
      <MainLayout>
        <SectionLayout>
          <h2 className="text-6xl font-bold text-indigo-500">posts</h2>
          <div className="flex flex-col gap-4 justify-center items-center w-full">
            {posts
              .sort(
                (a, b) =>
                  new Date(b.dateAdded).getTime() -
                  new Date(a.dateAdded).getTime()
              )
              .map((post) => (
                <BlogPostCard key={post.slug} {...post} />
              ))}
          </div>
        </SectionLayout>
      </MainLayout>
    </>
  );
}

async function getStaticProps(): Promise<GetStaticPropsResult<Props>> {
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

export default Index;
export { getStaticProps };
