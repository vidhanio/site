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
          bot developer, and a student at mcmaster. i'm currently working
          on a ton of cool projects, which you can find at 
          `}
          <A href="https://github.com/vidhanio">my github</A>.
        </p>
      </Section>
    </>
  );
}
