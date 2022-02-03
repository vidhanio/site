import Image from "next/image";

function Pre({ children }: { children?: React.ReactNode }): JSX.Element {
  return (
    <pre className="overflow-x-auto rounded-md bg-gray-200 px-0 py-4 font-mono text-gray-800 [font-feature-settings:normal]  dark:bg-gray-800 dark:text-gray-200">
      {children}
    </pre>
  );
}

function Img({ src, alt }: { src?: string; alt?: string }): JSX.Element {
  return (
    <div className="rounded-md">
      <Image
        src={src ?? ""}
        alt={alt}
        width={16}
        height={9}
        layout="responsive"
        objectFit="cover"
        className="rounded-md"
      />
      <figcaption>{alt}</figcaption>
    </div>
  );
}

const mdxComponents = {
  pre: Pre,
  img: Img,
};

export default mdxComponents;
