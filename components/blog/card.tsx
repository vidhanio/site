import Image from "next/image";
import Link from "next/link";
import { Post } from "types";

export default function Card(post: Post): JSX.Element {
  const dateAdded = new Date(post.dateAdded);

  const dateUpdated = post.dateUpdated ? new Date(post.dateUpdated) : undefined;

  return (
    <Link href={`/post/${post.slug}`}>
      <a className="flex w-full flex-col items-start justify-start rounded-md bg-gray-200 text-left text-indigo-500 shadow-lg dark:bg-gray-800 sm:w-96">
        {post.imageURL && (
          <div className="aspect-square h-auto w-full rounded-t-md">
            <Image
              src={post.imageURL}
              alt={post.title}
              width={1}
              height={1}
              layout="responsive"
              objectFit="cover"
              className="rounded-t-md"
            />
          </div>
        )}
        <div className="flex flex-col items-start justify-start p-4">
          <h2 className="text-2xl font-bold">{post.title}</h2>
          <h3 className="text-lg text-gray-900 dark:text-gray-100">
            {post.description}
          </h3>
          <time
            dateTime={dateAdded.toISOString()}
            className={
              "text-gray-800 dark:text-gray-200" +
              " " +
              (dateUpdated ? "text-sm line-through decoration-2" : "text-md")
            }
          >
            {dateAdded.toLocaleDateString("en-CA", {
              year: "numeric",
              month: "long",
              day: "numeric",
            })}
          </time>
          {dateUpdated && (
            <time
              dateTime={dateUpdated?.toISOString()}
              className="text-md text-gray-800 dark:text-gray-200"
            >
              Edited:{" "}
              {dateUpdated.toLocaleDateString("en-CA", {
                year: "numeric",
                month: "long",
                day: "numeric",
              })}
            </time>
          )}
        </div>
      </a>
    </Link>
  );
}
