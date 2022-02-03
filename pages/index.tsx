import Head from "next/head";
import { Section } from "components";

export default function Index() {
  return (
    <>
      <Head>
        <title>home - vidhan</title>
        <meta name="description" content="vidhan's home on the internet." />
      </Head>
      <h1 className="text-8xl font-extrabold text-indigo-500">vidhan</h1>
      <Section>
        <h2 className="text-4xl font-bold text-indigo-600 dark:text-indigo-400">
          introduction
        </h2>
        <p className="text-xl">
          hey! i'm vidhan. i'm a software engineer, fullstack developer, discord
          bot developer, and grade 12 high school student. i'm currently working
          on a ton of cool projects, which you can find at{" "}
          <a
            className="font-bold text-indigo-500 underline transition-colors  hover:text-emerald-500"
            target="_blank"
            href="https://github.com/vidhanio"
          >
            my github
          </a>
          .
        </p>
      </Section>
      <div></div>
    </>
  );
}
