import fs from "fs";
import matter from "gray-matter";
import { GetStaticPropsResult } from "next";
import { MDXRemote, MDXRemoteSerializeResult } from "next-mdx-remote";
import { serialize } from "next-mdx-remote/serialize";
import path from "path";
import { ParsedUrlQuery } from "querystring";
import { postFilePaths, POSTS_PATH } from "../../utils/mdx-utils";

interface Props {
  source: MDXRemoteSerializeResult<Record<string, unknown>>;
  frontMatter: {
    [key: string]: any;
  };
}
interface IParams extends ParsedUrlQuery {
  slug: string;
}

function PostPage({ source, frontMatter }: Props) {
  return (
    <div>
      <div>
        <h1>{frontMatter.title}</h1>
        {frontMatter.description ? (
          <p className="description">{frontMatter.description}</p>
        ) : null}
      </div>
      <main>
        <MDXRemote {...source} />
      </main>
    </div>
  );
}

async function getStaticProps({
  slug,
}: IParams): Promise<GetStaticPropsResult<Props>> {
  const postFilePath = path.join(POSTS_PATH, `${slug}.mdx`);
  const source = fs.readFileSync(postFilePath);

  const { data, content } = matter(source);

  const mdxSource = await serialize(content, {
    scope: data,
  });

  return {
    props: {
      source: mdxSource,
      frontMatter: data,
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

export default PostPage;
export { getStaticProps, getStaticPaths };
