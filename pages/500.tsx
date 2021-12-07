import MainLayout from "../components/layouts/main";
import H1 from "../components/headings/h1";
import H2 from "../components/headings/h2";

function Custom500(): JSX.Element {
  return (
    <MainLayout>
      <H1>{"500"}</H1>
      <H2>{"sorry, i suck at coding."}</H2>
    </MainLayout>
  );
}

export default Custom500;
