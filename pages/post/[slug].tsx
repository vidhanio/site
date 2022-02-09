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
      <header className="flex flex-col items-center justify-center gap-4 text-center">
        <h1 className="text-4xl font-extrabold italic text-indigo-500">
          {post.title}
        </h1>
        <p className="text-lg text-slate-700 dark:text-slate-300">
          {post.description}
        </p>

        <div className="flex flex-col">
          <time
            dateTime={dateAdded.toISOString()}
            className="text-md text-slate-600 dark:text-slate-400"
          >
            <a
              href={`https://github.com/vidhanio/site/commit/${post.hashAdded}`}
            >
              {dateAdded.toLocaleDateString("en-CA", {
                year: "numeric",
                month: "long",
                day: "numeric",
              })}
            </a>
          </time>

          {dateUpdated && (
            <time
              dateTime={dateUpdated?.toISOString()}
              className="text-sm text-slate-500"
            >
              <a
                href={`https://github.com/vidhanio/site/commit/${post.hashUpdated}`}
              >
                Edited:{" "}
                {dateUpdated.toLocaleDateString("en-CA", {
                  year: "numeric",
                  month: "long",
                  day: "numeric",
                })}
              </a>
            </time>
          )}
        </div>
      </header>

      <BlogArticleLayout>
        {post.imageURL && (
          <MDXImg src={post.imageURL} alt={post.imageAlt || post.title} />
        )}
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
