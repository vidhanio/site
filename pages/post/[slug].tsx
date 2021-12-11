import path from "path";

import { GetStaticPropsResult } from "next";
import { bundleMDX } from "mdx-bundler";
import { getMDXComponent } from "mdx-bundler/client";
import rehypePrism from "rehype-prism-plus";

import SEO from "@/seo";
import { postsPath, postSlugs } from "constants/posts";
import { mdxComponents } from "@/elements";
import { BlogMainLayout } from "layouts/blog";
import React from "react";
import Image from "next/image";

interface FrontmatterProps {
  title: string;
  description: string;
  imageURL?: string;
  dateAdded: Date;
  dateEdited?: Date;
}

interface Props {
  slug: string;
  code: string;
  frontmatter: {
    title: string;
    description: string;
    imageURL: string | null;
    dateAdded: [number, number, number];
    dateEdited: [number, number, number] | null;
  };
}

interface Params {
  params: {
    slug: string;
  };
}

function Post({ code, frontmatter }: Props) {
  const MDX = React.useMemo(() => getMDXComponent(code), [code]);

  const dateAdded = new Date(...frontmatter.dateAdded);

  const dateEdited = frontmatter.dateEdited
    ? new Date(...frontmatter.dateEdited)
    : undefined;
  return (
    <>
      <SEO
        title={frontmatter.title}
        description={frontmatter.description}
        imageURL={frontmatter.imageURL}
        slug={frontmatter.title}
      />
      <BlogMainLayout>
        <h1 className="mb-2 text-4xl text-indigo-500 md:text-6xl">
          {frontmatter.title}
        </h1>
        <h2 className="my-0 text-xl text-gray-800 dark:text-gray-200">
          {frontmatter.description}
        </h2>
        <time
          dateTime={dateAdded.toISOString()}
          className={dateEdited ? "line-through text-md" : "text-lg"}
        >
          {dateAdded.toLocaleDateString("en-CA", {
            year: "numeric",
            month: "long",
            day: "numeric",
          })}
        </time>
        {dateEdited && (
          <time dateTime={dateEdited?.toISOString()} className="text-lg">
            Edited:{" "}
            {dateEdited.toLocaleDateString("en-CA", {
              year: "numeric",
              month: "long",
              day: "numeric",
            })}
          </time>
        )}
        {frontmatter.imageURL && (
          <div className="mt-8 rounded-md shadow-md">
            <Image
              src={frontmatter.imageURL}
              alt={frontmatter.title}
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

  const { frontmatter, code } = await bundleMDX<FrontmatterProps>({
    file: filePath,
    xdmOptions(options) {
      options.rehypePlugins = [[rehypePrism, { showLineNumbers: true }]];
      return options;
    },
  });

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
    props: {
      slug: params.slug,
      code,
      frontmatter: {
        ...frontmatter,
        imageURL: frontmatter.imageURL ?? null,
        dateAdded,
        dateEdited,
      },
    },
  };
}

async function getStaticPaths() {
  const paths = postSlugs
    .map((path) => path.replace(/\.mdx?$/, ""))
    .map((slug) => ({ params: { slug } }));

  return {
    paths,
    fallback: false,
  };
}

export default Post;
export { getStaticProps, getStaticPaths };
