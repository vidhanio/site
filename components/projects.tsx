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
      <div className="flex w-full flex-col items-center justify-center rounded-xl bg-slate-200 dark:bg-slate-800 sm:w-96">
        {imageSrc ? (
          <Image
            src={imageSrc}
            alt={name}
            objectFit="cover"
            className="rounded-t-xl"
            width={384}
            height={384}
          />
        ) : (
          <div className="flex aspect-square w-full flex-col items-center justify-center sm:h-96 sm:w-96">
            <BeakerIcon className="h-16 w-16 fill-slate-300 dark:fill-slate-700" />
          </div>
        )}
        <div className="flex flex-col items-center justify-center p-4 text-center sm:p-8">
          <h3 className="text-xl font-bold text-indigo-600 dark:text-indigo-400">
            {name}
          </h3>
          <p className="text-slate-600 dark:text-slate-400">{description}</p>
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
    <div className="flex flex-row flex-wrap justify-center gap-4 sm:gap-8">
      {projects.map((project) => (
        <Project key={project.name} {...project} />
      ))}
    </div>
  );
}
