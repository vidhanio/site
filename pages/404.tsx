import MainLayout from "components/layouts/main";
import H1 from "components/headings/h1";
import H2 from "components/headings/h2";

function Custom404(): JSX.Element {
  return (
    <MainLayout>
      <H1>{"404"}</H1>
      <H2>{"better luck next time."}</H2>
    </MainLayout>
  );
}

export default Custom404;
