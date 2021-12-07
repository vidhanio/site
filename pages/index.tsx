import Layout from "../components/layout";
import Typewriter from "../components/typewriter";

function Index() {
  return (
    <Layout>
      <h1 className="text-8xl font-black">{"vidhan bhatt"}</h1>
      <h2 className="text-xl font-medium">
        <Typewriter
          className="font-bold text-green-600 dark:text-green-500"
          prefix="i'm a "
          strings={[
            "software developer",
            "discord bot developer",
            "frontend developer",
            "backend developer",
            "api developer",
            "full stack developer (?)",
          ]}
          suffix=" based in canada."
        />
      </h2>
    </Layout>
  );
}

export default Index;
