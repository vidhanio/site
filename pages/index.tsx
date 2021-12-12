import MainLayout from "layouts/main";
import Typewriter from "@/typewriter";

export default function Index() {
  return (
    <MainLayout>
      <h1 className="text-8xl font-black">{"vidhan bhatt"}</h1>
      <h2 className="text-2xl font-bold">
        <Typewriter
          className="font-bold text-emerald-500"
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
      </h2>
    </MainLayout>
  );
}
