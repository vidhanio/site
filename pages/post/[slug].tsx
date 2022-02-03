import mdxComponents, { MDXImg } from "components/blog/components";

import BlogArticleLayout from "layouts/blog/article";
import { GetStaticPropsResult } from "next";
import H1 from "components/elements/h1";
import { Post } from "types";
import React from "react";
import SEO from "components/blog/seo";
import { getMDXComponent } from "mdx-bundler/client";
import postFromSlug from "utils/post-from-slug";
import { postSlugs } from "constants/posts";

type Props = {
  post: Post;
};

type Params = {
  params: {
    slug: string;
  };
};

function PostPage({ post }: Props) {
  const MDX = React.useMemo(
    () => getMDXComponent(post.content),
    [post.content]
  );

  const dateAdded = new Date(post.dateAdded);
  const dateUpdated = post.dateUpdated ? new Date(post.dateUpdated) : undefined;

  return (
    <>
      <SEO post={post} />
      <BlogArticleLayout>
        <header className="flex flex-col">
          <H1 className="mb-2">{post.title}</H1>
          <p className="my-0 mt-2 text-lg text-gray-700 dark:text-gray-300">
            {post.description}
          </p>

          <time
            dateTime={dateAdded.toISOString()}
            className="text-md text-gray-600 dark:text-gray-400"
          >
            {dateAdded.toLocaleDateString("en-CA", {
              year: "numeric",
              month: "long",
              day: "numeric",
            })}
          </time>

          {dateUpdated && (
            <time
              dateTime={dateUpdated?.toISOString()}
              className="text-sm text-gray-500"
            >
              Edited:{" "}
              {dateUpdated.toLocaleDateString("en-CA", {
                year: "numeric",
                month: "long",
                day: "numeric",
              })}
            </time>
          )}

          {post.imageURL && <MDXImg src={post.imageURL} alt={post.title} />}
        </header>

        <MDX components={mdxComponents} />
      </BlogArticleLayout>
    </>
  );
}

async function getStaticProps({
  params,
}: Params): Promise<GetStaticPropsResult<Props>> {
  return {
    props: {
      post: await postFromSlug(params.slug),
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

export default PostPage;
export { getStaticProps, getStaticPaths };
