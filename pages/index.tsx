import Head from "next/head";
import Link from "components/link";
import { Section } from "components/section";

export default function Index() {
  return (
    <>
      <Head>
        <title>vidhan - home</title>
      </Head>
      <h1 className="text-8xl font-extrabold italic text-indigo-500">vidhan</h1>
      <Section title="introduction">
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
      <Section title="what am i workin on?">
        <p className="text-xl">
          <ul className="flex flex-col gap-4">
            <li>
              <h3 className="text-2xl">
                <Link href="https://github.com/vidhanio/diswordle">
                  diswordle
                </Link>
              </h3>
              a discord bot to play wordle right in your discord server.
            </li>
            <li>
              <h3 className="text-2xl">
                <Link href="https://github.com/vidhanio/checkpoint">
                  checkpoint
                </Link>
              </h3>
              a discord bot to provide easy verification for discord servers in
              my school board.
            </li>
            <li>
              <h3 className="text-2xl">
                <Link href="https://github.com/vidhanio/dmux">dmux</Link>
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
