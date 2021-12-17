import Image from "next/image";
import Link from "next/link";
import { motion } from "framer-motion";

function BlogPostCard(post: Post): JSX.Element {
  const dateAdded = new Date(post.dateAdded);

  const dateUpdated = post.dateUpdated ? new Date(post.dateUpdated) : undefined;

  return (
    <Link href={`/post/${post.slug}`} passHref>
      <motion.a
        whileHover={{ scale: 2 }}
        className="flex flex-col justify-start items-start w-full text-left text-indigo-500 bg-gray-200 rounded-md shadow-lg sm:w-96 dark:bg-gray-800"
      >
        {post.imageURL && (
          <div className="w-full h-auto rounded-t-md aspect-square">
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
        <div className="flex flex-col justify-start items-start p-4">
          <h2 className="text-2xl font-bold">{post.title}</h2>
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
      </motion.a>
    </Link>
  );
}

export default BlogPostCard;
