import Image from "next/image";
import Link from "next/link";

function BlogPostCard(post: Post): JSX.Element {
  const dateAdded = new Date(post.dateAdded);

  const dateUpdated = post.dateUpdated ? new Date(post.dateUpdated) : undefined;

  return (
    <Link href={`/post/${post.slug}`}>
      <a className="flex flex-col justify-start items-start w-full text-left text-indigo-500 bg-gray-200 rounded-md shadow-lg sm:items-center sm:h-32 sm:w-96 sm:flex-row dark:bg-gray-800">
        {post.imageURL && (
          <div className="relative w-full h-auto rounded-t-md sm:rounded-l-md sm:rounded-tr-none sm:w-auto sm:h-full aspect-square">
            <Image
              src={post.imageURL}
              alt={post.title}
              layout="fill"
              objectFit="cover"
              className="rounded-t-md sm:rounded-l-md sm:rounded-tr-none"
            />
          </div>
        )}
        <div className="flex flex-col justify-start items-start p-4">
          <h2 className="text-2xl font-bold text-ellipsis">{post.title}</h2>
          <h3 className="text-lg text-gray-900 dark:text-gray-100">
            {post.description}
          </h3>
          <time
            dateTime={dateAdded.toISOString()}
            className={
              "text-gray-800 dark:text-gray-200" +
              " " +
              (dateUpdated ? "line-through decoration-2 text-sm" : "text-md")
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
              className="text-gray-800 text-md dark:text-gray-200"
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

export default BlogPostCard;
