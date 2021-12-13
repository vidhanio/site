import { postSlugs, postsPath } from "constants/posts";

import { BlogMainLayout } from "layouts/blog";
import { GetStaticPropsResult } from "next";
import Image from "next/image";
import React from "react";
import SEO from "components/seo";
import { bundleMDX } from "mdx-bundler";
import { getMDXComponent } from "mdx-bundler/client";
import mdxComponents from "components/elements";
import path from "path";
import rehypePrism from "rehype-prism-plus";
import simpleGit from "simple-git";

interface Props {
  post: Post;
}

interface Params {
  params: {
    slug: string;
  };
}

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
          <div className="mt-8 rounded-md shadow-lg">
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
  );
}

async function getStaticProps({
  params,
}: Params): Promise<GetStaticPropsResult<Props>> {
  const filePath = path.join(postsPath, `${params.slug}.mdx`);

  const { frontmatter, code: content } = await bundleMDX<FrontmatterProps>({
    file: filePath,
    xdmOptions(options) {
      options.rehypePlugins = [[rehypePrism, { showLineNumbers: true }]];
      return options;
    },
  });

  const git = simpleGit();

  const commits = await git.log({
    file: filePath,
  });

  const firstCommit = commits.all[commits.all.length - 1];
  const lastCommit = commits.all[0];

  const dateAdded = firstCommit.date;
  const dateUpdated =
    firstCommit.hash !== lastCommit.hash ? lastCommit.date : null;

  const imageURL = frontmatter.imageURL ?? null;

  return {
    props: {
      post: {
        title: frontmatter.title,
        description: frontmatter.description,
        imageURL,
        slug: params.slug,
        content,
        dateAdded,
        dateUpdated,
      },
    },
  };
}

async function getStaticPaths() {
  const paths = postSlugs
    .map((file) => file.replace(/\.mdx?$/, ""))
    .map((slug) => ({ params: { slug } }));

  return {
    paths,
    fallback: false,
  };
}

export default Post;
export { getStaticProps, getStaticPaths };
