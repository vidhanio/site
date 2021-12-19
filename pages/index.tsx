import { GetServerSidePropsResult, InferGetServerSidePropsType } from "next";
import { MainLayout, SectionLayout } from "layouts/main";

import { Octokit } from "@octokit/rest";
import RepoCard from "components/repo-card";
import Typewriter from "components/typewriter";

type Props = {
  repos: Repository[];
};

export default function Index({
  repos,
}: InferGetServerSidePropsType<typeof getServerSideProps>) {
  return (
    <>
      <header className="flex flex-col gap-2 justify-center items-center w-full">
        <h1 className="text-8xl font-black text-indigo-500">vidhan bhatt</h1>
        <p className="text-2xl font-semibold text-indigo-700 dark:text-indigo-300">
          <Typewriter
            className="font-bold text-emerald-600 dark:text-emerald-400"
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
        </p>
      </header>
      <MainLayout>
        <SectionLayout>
          <h2 className="text-6xl font-bold text-indigo-500">who am i???</h2>
        </SectionLayout>
      </MainLayout>
    </>
  );
}

export async function getServerSideProps(): Promise<
  GetServerSidePropsResult<Props>
> {
  const octokit = new Octokit();
  const { data } = await octokit.repos.listForUser({
    username: "vidhanio",
  });
  const repos = data.map<Repository>((repo) => {
    return {
      name: repo.name,
      description: repo.description,
      topics: repo.topics === undefined ? [] : repo.topics,
      homepage: repo.homepage === undefined ? null : repo.homepage,
      language: repo.language === undefined ? "" : repo.language,
      url: repo.html_url,
      stars: repo.stargazers_count === undefined ? 0 : repo.stargazers_count,
      forks: repo.forks_count === undefined ? 0 : repo.forks_count,
      issues: repo.open_issues_count === undefined ? 0 : repo.open_issues_count,
    };
  });

  return {
    props: {
      repos,
    },
  };
}
