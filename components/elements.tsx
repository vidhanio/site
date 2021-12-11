import Image from "next/image";

function Pre({ children }: { children?: React.ReactNode }): JSX.Element {
  return (
    <pre className="overflow-x-auto shadow-lg px-0 text-gray-800 py-4 bg-gray-200 font-['Fira_Code'] rounded-md  dark:text-gray-200 dark:bg-gray-800">
      {children}
    </pre>
  );
}

function Img({ src, alt }: { src?: string; alt?: string }): JSX.Element {
  return (
    <div className="rounded-md shadow-md">
      <Image
        src={src as string}
        alt={alt}
        width={16}
        height={9}
        layout="responsive"
        objectFit="cover"
        className="rounded-md -z-10"
      />
    </div>
  );
}

const mdxComponents = {
  pre: Pre,
  img: Img,
};

export { Pre, mdxComponents };
