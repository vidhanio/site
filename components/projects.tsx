import { BeakerIcon } from "@heroicons/react/solid";
import Image from "next/image";

type ProjectProps = {
  name: string;
  description: string;
  href: string;
  imageSrc?: string;
};

export function Project({
  name,
  description,
  href,
  imageSrc,
}: ProjectProps): JSX.Element {
  return (
    <a href={href}>
      <div className="flex w-96 flex-col items-center justify-center rounded-xl bg-gray-200 dark:bg-gray-800">
        {imageSrc ? (
          <Image
            src={imageSrc}
            alt={name}
            objectFit="contain"
            className="rounded-t-xl"
            width={384}
            height={384}
          />
        ) : (
          <div className="flex h-96 w-96 flex-col items-center justify-center">
            <BeakerIcon className="h-16 w-16 fill-gray-300 dark:fill-gray-700" />
          </div>
        )}
        <div className="flex flex-col items-center justify-center p-8 text-center">
          <h3 className="text-xl font-bold text-indigo-600 dark:text-indigo-400">
            {name}
          </h3>
          <p className="text-gray-600 dark:text-gray-400">{description}</p>
        </div>
      </div>
    </a>
  );
}

type Props = {
  projects: ProjectProps[];
};

export default function Projects({ projects }: Props): JSX.Element {
  return (
    <div className="flex flex-row flex-wrap justify-center gap-8">
      {projects.map((project) => (
        <Project key={project.name} {...project} />
      ))}
    </div>
  );
}
