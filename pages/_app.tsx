import Head from "next/head";
import type { AppProps } from "next/app";

import "tailwindcss/tailwind.css";

import Nav from "components/nav/nav";

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

export default CustomApp;
