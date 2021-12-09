import path from "path";

import { GetStaticPropsResult } from "next";
import { bundleMDX } from "mdx-bundler";
import { getMDXComponent } from "mdx-bundler/client";
import rehypeHighlight from "rehype-highlight";

import { postPath, postFilePaths } from "utils/mdx";
import { mdxComponents } from "@/mdx";
import { BlogHeaderLayout, BlogMainLayout } from "layouts/blog";
import { H1, H2, H3 } from "@/elements/headings";
import React from "react";

interface Props {
  code: string;
  frontmatter: {
    title: string;
    description: string;
    addedDate: number;
    editedDate?: number;
  };
}

interface Params {
  params: {
    slug: string;
  };
}

function Post({ code, frontmatter }: Props) {
  const MDX = React.useMemo(() => getMDXComponent(code), [code]);

  const addedDate = new Date(frontmatter.addedDate * 1000).toLocaleDateString(
    "en-CA",
    {
      year: "numeric",
      month: "long",
      day: "numeric",
    }
  );

  const editedDate =
    frontmatter.editedDate && frontmatter.editedDate !== 0
      ? new Date(frontmatter.editedDate * 1000).toLocaleDateString("en-CA", {
          year: "numeric",
          month: "long",
          day: "numeric",
        })
      : undefined;
  return (
    <>
      <BlogHeaderLayout>
        <H1>{frontmatter.title}</H1>
        <H2>{frontmatter.description}</H2>
        <H3 className={editedDate && "line-through text-lg"}>{addedDate}</H3>
        <H3>{editedDate}</H3>
      </BlogHeaderLayout>
      <BlogMainLayout>
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
      options.rehypePlugins = [rehypeHighlight];
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
