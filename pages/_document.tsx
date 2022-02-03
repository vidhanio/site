import { Head, Html, Main, NextScript } from "next/document";

export default function Document(): JSX.Element {
  return (
    <Html lang="en">
      <Head />
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
