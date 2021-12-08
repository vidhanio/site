import MainLayout from "@/layouts/main";
import { H1, H2 } from "@/elements/headings";

function Custom404(): JSX.Element {
  return (
    <MainLayout>
      <H1>{"404"}</H1>
      <H2>{"better luck next time."}</H2>
    </MainLayout>
  );
}

export default Custom404;
