import { GetStaticPropsResult, InferGetStaticPropsType } from "next";
import { MainLayout, SectionLayout } from "layouts/main";
import { postSlugs, postsPath } from "constants/posts";

import BlogCard from "components/blog-card";
import { bundleMDX } from "mdx-bundler";
import path from "path";
import rehypePrism from "rehype-prism-plus";
import simpleGit from "simple-git";

interface Props {
  posts: Post[];
}

function Index({ posts }: InferGetStaticPropsType<typeof getStaticProps>) {
  return (
    <>
      <header className="flex flex-col gap-2 justify-center items-center w-full">
        <h1 className="text-8xl font-black text-indigo-500">
          {"vidhan's blog"}
        </h1>
      </header>
      <MainLayout>
        <SectionLayout>
          <h2 className="text-6xl font-bold text-indigo-500">posts</h2>
          <div className="flex flex-col gap-4 justify-center items-center w-full">
            {posts
              .sort(
                (a, b) =>
                  new Date(b.dateAdded).getTime() -
                  new Date(a.dateAdded).getTime()
              )
              .map((post) => (
                <BlogCard key={post.slug} {...post} />
              ))}
          </div>
        </SectionLayout>
      </MainLayout>
    </>
  );
}

async function getStaticProps(): Promise<GetStaticPropsResult<Props>> {
  const postsPromise = postSlugs.map(async (slug): Promise<Post> => {
    const filePath = path.join(postsPath, `${slug}.mdx`);

    const { frontmatter, code: content } = await bundleMDX<FrontmatterProps>({
      file: filePath,
      xdmOptions(options) {
        options.rehypePlugins = [[rehypePrism, { showLineNumbers: true }]];
        return options;
      },
    });

    const git = simpleGit();

    const commits = await git.log({
      file: filePath,
    });

    const firstCommit = commits.all[commits.all.length - 1];
    const lastCommit = commits.all[0];

    const dateAdded = firstCommit.date;
    const dateUpdated =
      firstCommit.hash !== lastCommit.hash ? lastCommit.date : null;

    const imageURL = frontmatter.imageURL ?? null;

    return {
      title: frontmatter.title,
      description: frontmatter.description,
      imageURL,
      slug,
      content,
      dateAdded,
      dateUpdated,
    };
  });

  return {
    props: {
      posts: await Promise.all(postsPromise),
    },
  };
}

export default Index;
export { getStaticProps };
