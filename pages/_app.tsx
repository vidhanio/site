import type { AppProps } from "next/app";
import Head from "next/head";

import "tailwindcss/tailwind.css";

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <>
      <Head>
        <title>vidhan - blog</title>
        <link rel="icon" href="favicon.ico" />

        <meta property="og:title" content="vidhan - blog" />
        <meta property="og:type" content="website" />
        <meta property="og:url" content="https://blog.vidhan.io" />
        <meta property="og:image" content="https://vidhan.io/og.png" />
        <meta property="og:description" content="💭 → 📄" />
      </Head>
      <Component {...pageProps} />
    </>
  );
}

export default MyApp;
