import Nav from "../components/nav";
import Subheading from "../components/subheading";

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
        <Subheading
          prefix="i'm a "
          suffix=" based in canada."
          words={["software developer", "full stack developer", "coder"]}
        />
      </div>
    </>
  );
}

export default Index;
