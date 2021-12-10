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
          href="https://fonts.googleapis.com/css2?family=Fira+Code:wght@300;400;500;600;700&family=Playfair+Display:ital,wght@0,400;0,500;0,600;0,700;0,800;0,900;1,400;1,500;1,600;1,700;1,800;1,900&display=swap"
          rel="stylesheet"
        />
      </Head>
      <body
        className={`text-violet-500 bg-gray-100 font-['Playfair_Display'] dark:bg-gray-900`}
      >
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}

export default CustomDocument;
