import { Html, Head, Main, NextScript, DocumentProps } from "next/document";

function CustomDocument(_props: DocumentProps): JSX.Element {
  return (
    <Html lang="en">
      <Head />
      <body
        className={`font-serif text-purple-600 bg-gray-100 dark:text-purple-500 dark:bg-gray-900`}
      >
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}

export default CustomDocument;
