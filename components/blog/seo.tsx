import Head from "next/head";
import { Post } from "types";

type Props = {
  post: Post;
};

function SEO({ post }: Props) {
  const title = `vidhan - blog: ${post.title}`;
  const url = `https://vidhan.io/post/${post.slug}`;

  return (
    <Head>
      <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      <meta charSet="UTF-8" />
      <meta httpEquiv="X-UA-Compatible" content="IE=edge" />

      <title>{title}</title>
      <meta name="description" content={post.description} />
      <meta name="theme-color" content="#6466e9" />
      <link rel="icon" href="/favicon.ico" />

      <meta name="og:title" content={title} />
      <meta name="og:description" content={post.description} />
      <meta name="og:image" content={post.imageURL ?? ""} />
      <meta name="og:url" content={url} />
      <meta name="og:type" content="website" />

      <meta name="twitter:card" content="summary_large_image" />
      <meta name="twitter:site" content="@vidhanio" />
      <meta name="twitter:creator" content="@vidhanio" />
      <meta name="twitter:title" content={title} />
      <meta name="twitter:description" content={post.description} />
      <meta name="twitter:image" content={post.imageURL ?? ""} />
      <meta name="og:url" content={url} />
    </Head>
  );
}

export default SEO;
