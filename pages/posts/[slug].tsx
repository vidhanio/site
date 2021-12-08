import fs from "fs";
import path from "path";

import { GetStaticPropsResult } from "next";
import { serialize } from "next-mdx-remote/serialize";
import { MDXRemote, MDXRemoteSerializeResult } from "next-mdx-remote";
import matter from "gray-matter";

import { postsPath, postFilePaths } from "utils/mdx";
import { postComponents } from "@/mdx";
import { H1 } from "@/elements/headings";
import { BlogHeaderLayout, BlogMainLayout } from "@/layouts/blog";

interface Props {
  content: MDXRemoteSerializeResult<Record<string, unknown>>;
  frontmatter: {
    [key: string]: any;
  };
}

interface Params {
  params: {
    slug: string;
  };
}

function Post({ content, frontmatter }: Props) {
  return (
    <>
      <BlogHeaderLayout>
        <H1>{frontmatter.title}</H1>
        {frontmatter.description ?? (
          <p className="description">{frontmatter.description}</p>
        )}
      </BlogHeaderLayout>
      <BlogMainLayout>
        <MDXRemote {...content} components={postComponents} />
      </BlogMainLayout>
    </>
  );
}

async function getStaticProps({
  params,
}: Params): Promise<GetStaticPropsResult<Props>> {
  const postFilePath = path.join(postsPath, `${params.slug}.mdx`);
  const source = fs.readFileSync(postFilePath);

  const { data, content } = matter(source);

  const mdxSource = await serialize(content, {
    scope: data,
  });

  return {
    props: {
      content: mdxSource,
      frontmatter: data,
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
