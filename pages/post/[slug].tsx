import { BlogMainLayout } from "layouts/blog";
import { GetStaticPropsResult } from "next";
import Image from "next/image";
import PostFromSlug from "utils/post-from-slug";
import React from "react";
import SEO from "components/seo";
import { getMDXComponent } from "mdx-bundler/client";
import mdxComponents from "components/elements";
import { postSlugs } from "constants/posts";

type Props = {
  post: Post;
};

type Params = {
  params: {
    slug: string;
  };
};

function Post({ post }: Props) {
  const MDX = React.useMemo(
    () => getMDXComponent(post.content),
    [post.content]
  );

  const dateAdded = new Date(post.dateAdded);
  const dateUpdated = post.dateUpdated ? new Date(post.dateUpdated) : undefined;

  return (
    <>
      <SEO {...post} />
      <>
        <BlogMainLayout>
          <header className="flex flex-col">
            <h1 className="mb-2 text-4xl text-indigo-500 md:text-6xl">
              {post.title}
            </h1>
            <h2 className="my-0 text-xl text-gray-800 dark:text-gray-200">
              {post.description}
            </h2>
            <time
              dateTime={dateAdded.toISOString()}
              className={dateUpdated ? "line-through text-md" : "text-lg"}
            >
              {dateAdded.toLocaleDateString("en-CA", {
                year: "numeric",
                month: "long",
                day: "numeric",
              })}
            </time>
          </header>
          {dateUpdated && (
            <time dateTime={dateUpdated?.toISOString()} className="text-lg">
              Edited:{" "}
              {dateUpdated.toLocaleDateString("en-CA", {
                year: "numeric",
                month: "long",
                day: "numeric",
              })}
            </time>
          )}
          {post.imageURL && (
            <div className="mt-8 rounded-md shadow-lg aspect-video">
              <Image
                src={post.imageURL}
                alt={post.title}
                width={16}
                height={9}
                layout="responsive"
                objectFit="cover"
                className="rounded-md -z-10"
              />
            </div>
          )}
          <MDX components={mdxComponents} />
        </BlogMainLayout>
      </>
    </>
  );
}

async function getStaticProps({
  params,
}: Params): Promise<GetStaticPropsResult<Props>> {
  return {
    props: {
      post: await PostFromSlug(params.slug),
    },
  };
}

async function getStaticPaths() {
  const paths = postSlugs
    .map<string>((file) => file.replace(/\.mdx?$/, ""))
    .map<Params>((slug) => ({ params: { slug } }));

  return {
    paths,
    fallback: false,
  };
}

export default Post;
export { getStaticProps, getStaticPaths };
