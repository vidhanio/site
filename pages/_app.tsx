import "styles/globals.css";

import type { AppProps } from "next/app";
import Head from "next/head";
import Nav from "components/nav";
import { WrapperLayout } from "layouts/global";

export default function App({ Component, pageProps }: AppProps): JSX.Element {
  return (
    <>
      <Head>
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <meta charSet="UTF-8" />
        <meta httpEquiv="X-UA-Compatible" content="IE=edge" />

        <meta name="description" content="vidhan's home on the internet." />
        <meta name="og:title" content="vidhan" />
        <meta name="og:description" content="vidhan's home on the internet." />
        <meta name="og:image" content="https://vidhan.io/images/og-image.png" />
        <meta name="og:url" content="https://vidhan.io" />
        <meta name="og:type" content="website" />

        <meta name="twitter:card" content="summary_large_image" />
        <meta name="twitter:site" content="@vidhanio" />
        <meta name="twitter:creator" content="@vidhanio" />
        <meta name="twitter:title" content="vidhan" />
        <meta
          name="twitter:description"
          content="vidhan's home on the internet."
        />
        <meta
          name="twitter:image"
          content="https://vidhan.io/images/og-image.png"
        />
        <meta name="twitter:url" content="https://vidhan.io" />

        <meta name="theme-color" content="#6466e9" />

        <link rel="icon" href="/favicon.ico" />
      </Head>
      <Nav
        navItems={[
          {
            name: "home",
            url: "/",
          },
          {
            name: "projects",
            url: "/projects",
          },
          {
            name: "blog",
            url: "https://blog.vidhan.io",
          },
        ]}
      />
      <WrapperLayout>
        <Component {...pageProps} />
      </WrapperLayout>
    </>
  );
}
