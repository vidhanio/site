import Head from "next/head";
import { Projects } from "components/projects";

export default function ProjectsPage() {
  return (
    <>
      <Head>
        <title>vidhan - projects</title>
      </Head>
      <h1 className="text-8xl font-extrabold italic text-indigo-500">
        projects
      </h1>
      <Projects
        projects={[
          {
            name: "diswordle",
            description:
              "a discord bot to play wordle right in your discord server.",
            href: "https://github.com/vidhanio/diswordle",
          },
          {
            name: "checkpoint",
            description:
              "a discord bot to provide easy verification for discord servers in my school board.",
            href: "https://github.com/vidhanio/checkpoint",
          },
          {
            name: "dmux",
            description:
              "a package for go to make discord command definitions more declarative.",
            href: "https://github.com/vidhanio/dmux",
          },
        ]}
      />
    </>
  );
}
