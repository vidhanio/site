import {
  faCodeBranch,
  faDotCircle,
  faStar,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import Link from "next/link";

export default function RepoCard(repo: Repository): JSX.Element {
  return (
    <Link href={repo.url}>
      <a className="flex flex-col gap-4 justify-center items-center p-4 w-72 h-72 bg-gray-200 rounded-md shadow-lg dark:bg-gray-800">
        <div className="flex flex-col gap-1">
          <h2 className="text-lg font-bold text-indigo-600 dark:text-indigo-400">
            {repo.name}
          </h2>
          {repo.description && (
            <p className="text-sm text-gray-700 dark:text-gray-300">
              {repo.description}
            </p>
          )}
        </div>
        <div className="flex flex-row justify-evenly w-full text-gray-900 dark:text-gray-100">
          <div className="flex flex-row gap-2 justify-center items-center">
            <FontAwesomeIcon icon={faCodeBranch} />
            <p>{repo.forks}</p>
          </div>
          <div className="flex flex-row gap-2 justify-center items-center">
            <FontAwesomeIcon icon={faStar} />
            <p>{repo.stars}</p>
          </div>
          <div className="flex flex-row gap-2 justify-center items-center">
            <FontAwesomeIcon icon={faDotCircle} />
            <p>{repo.issues}</p>
          </div>
        </div>
        {repo.topics && (
          <div className="flex overflow-scroll flex-row flex-wrap gap-2 justify-center items-center">
            {repo.topics.map((topic) => (
              <span
                key={topic}
                className="p-1 text-xs text-gray-700 bg-gray-300 rounded-md dark:bg-gray-700 dark:text-gray-300"
              >
                {topic}
              </span>
            ))}
          </div>
        )}
      </a>
    </Link>
  );
}
