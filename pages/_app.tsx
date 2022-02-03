import "styles/globals.css";
import "styles/prism.css";

import type { AppProps } from "next/app";
import Nav from "components/nav";
import { WrapperLayout } from "layouts/wrapper";

export default function App({ Component, pageProps }: AppProps): JSX.Element {
  return (
    <>
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
            url: "/blog",
          },
        ]}
      />
      <WrapperLayout>
        <Component {...pageProps} />
      </WrapperLayout>
    </>
  );
}
