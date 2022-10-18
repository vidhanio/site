import Image from "next/image";
import Link from "next/link";
import { NewspaperIcon } from "@heroicons/react/24/solid";
import { Post } from "types";

type PostCardProps = {
  post: Post;
};

export function PostCard({ post }: PostCardProps): JSX.Element {
  const dateAdded = new Date(post.dateAdded);

  return (
    <Link href={`/post/${post.slug}`}>
      <a>
        <div className="flex w-full flex-col items-center justify-center rounded-xl bg-slate-200 dark:bg-slate-800 sm:w-96">
          {post.imageURL ? (
            <Image
              src={post.imageURL}
              alt={post.title}
              objectFit="cover"
              className="rounded-t-xl"
              width={384}
              height={384}
            />
          ) : (
            <div className="flex aspect-square w-full flex-col items-center justify-center sm:h-96 sm:w-96">
              <NewspaperIcon className="h-16 w-16 fill-slate-300 dark:fill-slate-700" />
            </div>
          )}
          <div className="flex flex-col items-center justify-center p-4 text-center sm:p-8">
            <h3 className="text-xl font-bold text-indigo-600 dark:text-indigo-400">
              {post.title}
            </h3>
            <p className="text-slate-600 dark:text-slate-400">
              {post.description}
            </p>
            <time
              dateTime={dateAdded.toISOString()}
              className="text-slate-800 dark:text-slate-500"
            >
              {dateAdded.toLocaleDateString("en-CA", {
                year: "numeric",
                month: "long",
                day: "numeric",
              })}
            </time>
          </div>
        </div>
      </a>
    </Link>
  );
}

type Props = {
  posts: Post[];
};

export default function Posts({ posts }: Props): JSX.Element {
  return (
    <div className="flex flex-row flex-wrap justify-center gap-4 sm:gap-8">
      {posts
        .sort(
          (a, b) =>
            new Date(b.dateAdded).getTime() - new Date(a.dateAdded).getTime()
        )
        .map((post) => (
          <PostCard key={post.slug} post={post} />
        ))}
    </div>
  );
}
