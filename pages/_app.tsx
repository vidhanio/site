import type { AppProps } from "next/app";
import Head from "next/head";

import "tailwindcss/tailwind.css";
import "../styles/main.css";

import Nav from "../components/nav";

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <>
      <Head>
        <title>vidhan</title>
        <link rel="icon" href="favicon.ico" />

        <meta property="og:title" content="vidhan" />
        <meta property="og:type" content="website" />
        <meta property="og:url" content="https://vidhan.io" />
        <meta property="og:image" content="https://vidhan.io/og.png" />
        <meta
          property="og:description"
          content="vidhan's home on the internet."
        />
      </Head>
      <Nav
        navItems={[
          {
            name: "home",
            url: "/",
          },
          {
            name: "blog",
            url: "https://blog.vidhan.io",
          },
          {
            name: "resume",
            url: "/resume",
          },
          {
            name: "triple-tac-toe",
            url: "https://triple-tac-toe.vidhan.io",
          },
        ]}
      />
      <Component {...pageProps} />
    </>
  );
}

export default MyApp;
