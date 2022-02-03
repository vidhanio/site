import A from "components/elements/a";
import H1 from "components/elements/h1";
import SEO from "components/seo";
import { Section } from "components/section";

export default function IndexPage() {
  return (
    <>
      <SEO />

      <H1>vidhan</H1>

      <Section title="introduction">
        <p className="text-xl">
          {`
          hey! i'm vidhan. i'm a software engineer, fullstack developer, discord
          bot developer, and grade 12 high school student. i'm currently working
          on a ton of cool projects, which you can find at 
          `}
          <A href="https://github.com/vidhanio">my github</A>.
        </p>
      </Section>

      <Section title="what am i workin on?">
        <p className="text-xl">
          <ul className="flex flex-col gap-4">
            <li>
              <h3 className="text-2xl">
                <A href="https://github.com/vidhanio/diswordle">diswordle</A>
              </h3>
              a discord bot to play wordle right in your discord server.
            </li>
            <li>
              <h3 className="text-2xl">
                <A href="https://github.com/vidhanio/checkpoint">checkpoint</A>
              </h3>
              a discord bot to provide easy verification for discord servers in
              my school board.
            </li>
            <li>
              <h3 className="text-2xl">
                <A href="https://github.com/vidhanio/dmux">dmux</A>
              </h3>
              a package for go to make discord command definitions more
              declarative.
            </li>
          </ul>
        </p>
      </Section>
    </>
  );
}
