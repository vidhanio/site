import MainLayout from "@/layouts/main";
import { H1, H2 } from "@/elements/headings";

function Custom500(): JSX.Element {
  return (
    <MainLayout>
      <H1>{"500"}</H1>
      <H2>{"sorry, i suck at coding."}</H2>
    </MainLayout>
  );
}

export default Custom500;
