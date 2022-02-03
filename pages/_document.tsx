import { Head, Html, Main, NextScript } from "next/document";

export default function Document(): JSX.Element {
  return (
    <Html lang="en">
      <Head>
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
      <body
        className={
          "bg-gray-100 font-mono text-gray-900 [font-feature-settings:'ss05'] dark:bg-gray-900 dark:text-gray-100"
        }
      >
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}
