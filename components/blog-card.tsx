import Image from "next/image";
import Link from "next/link";

interface CardProps extends SEOProps {
  dateAdded: [number, number, number];
  dateEdited: [number, number, number] | null;
}

function BlogCard(card: CardProps): JSX.Element {
  const dateAdded = new Date(...card.dateAdded);

  const dateEdited = card.dateEdited ? new Date(...card.dateEdited) : undefined;
  return (
    <Link href={`/post/${card.slug}`}>
      <a className="flex flex-col justify-start items-start w-full text-left text-indigo-500 bg-gray-200 rounded-md shadow-md sm:items-center sm:h-32 sm:w-96 sm:flex-row dark:bg-gray-800">
        {card.imageURL && (
          <div className="relative w-full h-auto rounded-t-md sm:rounded-l-md sm:rounded-tr-none sm:w-auto sm:h-full aspect-square">
            <Image
              src={card.imageURL}
              alt={card.title}
              layout="fill"
              objectFit="cover"
              className="rounded-t-md sm:rounded-l-md sm:rounded-tr-none"
            />
          </div>
        )}
        <div className="flex flex-col justify-start items-start p-4">
          <h2 className="text-2xl font-bold text-ellipsis">{card.title}</h2>
          <h3 className="text-lg text-gray-900 dark:text-gray-100">
            {card.description}
          </h3>
          <time
            dateTime={dateAdded.toISOString()}
            className={
              "text-gray-800 dark:text-gray-200" +
              " " +
              (dateEdited ? "line-through decoration-2 text-sm" : "text-md")
            }
          >
            {dateAdded.toLocaleDateString("en-CA", {
              year: "numeric",
              month: "long",
              day: "numeric",
            })}
          </time>
          {dateEdited && (
            <time
              dateTime={dateEdited?.toISOString()}
              className="text-gray-800 text-md dark:text-gray-200"
            >
              Edited:{" "}
              {dateEdited.toLocaleDateString("en-CA", {
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

export default BlogCard;
