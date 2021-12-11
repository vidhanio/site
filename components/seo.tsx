import Head from "next/head";

interface Props {
  title: string;
  description: string;
  imageURL: string | null;
  slug: string;
}

function SEO({ title, description, imageURL, slug }: Props) {
  return (
    <Head>
      <title>{title}</title>
      <meta name="description" content={description} />

      <meta property="og:title" content={title} />
      <meta property="og:description" content={description} />
      {imageURL && <meta property="og:image" content={imageURL} />}
      <meta property="og:type" content="website" />
      <meta property="og:url" content={`https://blog.vidhan.io/post/${slug}`} />

      <meta name="twitter:card" content="summary_large_image" />
      <meta name="twitter:title" content={title} />
      <meta name="twitter:description" content={description} />
      {imageURL && <meta name="twitter:image" content={imageURL} />}
      <meta name="twitter:image:alt" content={title} />
      <meta
        name="twitter:url"
        content={`https://blog.vidhan.io/post/${slug}`}
      />
    </Head>
  );
}

export default SEO;
