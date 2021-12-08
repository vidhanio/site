import fs from "fs";
import { ParsedUrlQuery } from "querystring";
import path from "path";

import { GetStaticPathsResult, GetStaticPropsResult } from "next";
import { serialize } from "next-mdx-remote/serialize";
import { MDXRemote, MDXRemoteSerializeResult } from "next-mdx-remote";
import matter from "gray-matter";

import { postsPath, postFilePaths } from "../../utils/mdx-utils";
import { postComponents } from "../../components/post/components";
import H1 from "../../components/headings/h1";
import BlogHeaderLayout from "../../components/layouts/blog/header";
import BlogMainLayout from "../../components/layouts/blog/main";

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
