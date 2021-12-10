import path from "path";

import { GetStaticPropsResult } from "next";
import { bundleMDX } from "mdx-bundler";
import { getMDXComponent } from "mdx-bundler/client";
import rehypePrism from "rehype-prism-plus";

import { postPath, postFilePaths } from "constants/posts";
import { H1, H2, H3, mdxComponents } from "@/elements";
import { BlogMainLayout } from "layouts/blog";
import React from "react";

interface Props {
  code: string;
  frontmatter: {
    title: string;
    description: string;
    dateAdded: number;
    dateEdited?: number;
  };
}

interface Params {
  params: {
    slug: string;
  };
}

function Post({ code, frontmatter }: Props) {
  const MDX = React.useMemo(() => getMDXComponent(code), [code]);

  const dateAdded = new Date(frontmatter.dateAdded * 1000).toLocaleDateString(
    "en-CA",
    {
      year: "numeric",
      month: "long",
      day: "numeric",
    }
  );

  const dateEdited =
    frontmatter.dateEdited && frontmatter.dateEdited !== 0
      ? new Date(frontmatter.dateEdited * 1000).toLocaleDateString("en-CA", {
          year: "numeric",
          month: "long",
          day: "numeric",
        })
      : undefined;
  return (
    <>
      <BlogMainLayout>
        <h1 className="mb-2">{frontmatter.title}</h1>
        <p className="my-0 text-xl text-gray-800 dark:text-gray-200">
          {frontmatter.description}
        </p>
        <p
          className={"mt-0 text-lg" + " " + (dateEdited ? "line-through" : "")}
        >
          {dateAdded}
        </p>
        {dateEdited && <p className="mt-2 text-xl">Edited: {dateEdited}</p>}
        <MDX components={mdxComponents} />
      </BlogMainLayout>
    </>
  );
}

async function getStaticProps({
  params,
}: Params): Promise<GetStaticPropsResult<Props>> {
  const filePath = path.join(postPath, `${params.slug}.mdx`);

  const { frontmatter, code } = await bundleMDX<Props["frontmatter"]>({
    file: filePath,
    xdmOptions(options) {
      options.rehypePlugins = [[rehypePrism, { showLineNumbers: true }]];

      return options;
    },
  });

  return {
    props: {
      code,
      frontmatter: frontmatter,
    },
  };
}

async function getStaticPaths() {
  const paths = postFilePaths
    .map((path) => path.replace(/\.mdx?$/, ""))
    .map((slug) => ({ params: { slug } }));

  return {
    paths,
    fallback: false,
  };
}

export default Post;
export { getStaticProps, getStaticPaths };
