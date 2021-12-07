import Nav from "../components/nav";
import Typewriter from "../components/typewriter";

function Index() {
  return (
    <>
      <Nav
        navItems={[
          {
            name: "home",
            url: "/",
          },
          {
            name: "blog",
            url: "https://blog.vidhan.io",
          },
          {
            name: "resume",
            url: "/resume",
          },
          {
            name: "triple-tac-toe",
            url: "https://triple-tac-toe.vidhan.io",
          },
        ]}
      />
      <div className="flex flex-col gap-2 justify-center items-center p-16 w-full h-screen">
        <h1 className="text-8xl font-black">{"vidhan bhatt"}</h1>
        <h2 className="text-xl font-medium">
          <Typewriter
            className="font-bold text-green-600 dark:text-green-500"
            prefix="i'm a "
            strings={["software developer", "full stack developer", "coder"]}
            suffix=" based in canada."
          />
        </h2>
      </div>
    </>
  );
}

export default Index;
