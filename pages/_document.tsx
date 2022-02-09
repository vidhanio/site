import { Head, Html, Main, NextScript } from "next/document";

export default function Document(): JSX.Element {
  return (
    <Html lang="en">
      <Head />
      <body
        className={
          "bg-slate-100 font-mono text-slate-900 [font-feature-settings:'ss05'] dark:bg-slate-900 dark:text-slate-100"
        }
      >
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}
