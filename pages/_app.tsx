import Head from "next/head";
import type { AppProps } from "next/app";

import "tailwindcss/tailwind.css";
import "styles/prism.css";

import Nav from "@/nav";

function CustomApp({ Component, pageProps }: AppProps): JSX.Element {
  return (
    <>
      <Head>
        <meta charSet="UTF-8" />
        <meta httpEquiv="X-UA-Compatible" content="IE=edge" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      </Head>
      <Nav
        navItems={[
          {
            name: "home",
            url: "https://vidhan.io",
          },
          {
            name: "blog",
            url: "/",
          },
          {
            name: "resume",
            url: "https://vidhan.io/resume",
          },
        ]}
      />
      <Component {...pageProps} />
    </>
  );
}

export default CustomApp;
