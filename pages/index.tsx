import MainLayout from "@/layouts/main";
import { H1, H2 } from "@/elements/headings";
import Typewriter from "@/misc/typewriter";

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
