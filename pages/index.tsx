import MainLayout from "components/layouts/main";
import H1 from "components/headings/h1";
import H2 from "components/headings/h2";
import Typewriter from "components/typewriter";

function Index(): JSX.Element {
  return (
    <>
      <MainLayout>
        <H1>{"vidhan bhatt"}</H1>
        <H2>
          <Typewriter
            className="font-bold text-green-500"
            prefix="i'm a "
            prefixVowel="i'm an "
            strings={[
              "high school student",
              "software developer",
              "discord bot developer",
              "frontend developer",
              "backend developer",
              "api developer",
              "full stack developer (?)",
            ]}
            suffix=" from canada."
          />
        </H2>
      </MainLayout>
    </>
  );
}

export default Index;
