import Head from "next/head";

function SEO(post: Post) {
  return (
    <Head>
      <title>{post.title}</title>
      <meta name="description" content={post.description} />

      <meta property="og:title" content={post.title} />
      <meta property="og:description" content={post.description} />
      {post.imageURL && <meta property="og:image" content={post.imageURL} />}
      <meta property="og:type" content="website" />
      <meta
        property="og:url"
        content={`https://blog.vidhan.io/post/${post.slug}`}
      />

      <meta name="twitter:card" content="summary_large_image" />
      <meta name="twitter:title" content={post.title} />
      <meta name="twitter:description" content={post.description} />
      {post.imageURL && <meta name="twitter:image" content={post.imageURL} />}
      <meta name="twitter:image:alt" content={post.title} />
      <meta
        name="twitter:url"
        content={`https://blog.vidhan.io/post/${post.slug}`}
      />
    </Head>
  );
}

export default SEO;
