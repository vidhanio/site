import { Html, Head, Main, NextScript, DocumentProps } from "next/document";

function CustomDocument(_props: DocumentProps): JSX.Element {
  return (
    <Html lang="en">
      <Head>
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link
          rel="preconnect"
          href="https://fonts.gstatic.com"
          crossOrigin="anonymous"
        />
        {/*eslint-disable-next-line @next/next/no-page-custom-font*/}
        <link
          href="https://fonts.googleapis.com/css2?family=Poppins:wght@400;700;900&display=swap"
          rel="stylesheet"
        />
      </Head>
      <body
        className={`font-sans text-purple-500 bg-gray-100 dark:bg-gray-900`}
      >
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}

export default CustomDocument;
