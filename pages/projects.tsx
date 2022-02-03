import H1 from "components/elements/h1";
import Projects from "components/projects";
import SEO from "components/seo";

export default function ProjectsPage() {
  return (
    <>
      <SEO path="projects" />
      <H1>projects</H1>
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
