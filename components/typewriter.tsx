import { useState, useEffect } from "react";

function Typewriter({
  prefix,
  strings,
  suffix,
  className,
}: {
  prefix?: string;
  strings: string[];
  suffix?: string;
  className?: string;
}) {
  const [index, setIndex] = useState<number>(0);
  const [subIndex, setSubIndex] = useState<number>(0);
  const [reverse, setReverse] = useState<boolean>(false);

  useEffect(() => {
    if (subIndex === strings[index].length + 1 && !reverse) {
      setReverse(true);
      return;
    }

    if (subIndex === 0 && reverse) {
      setReverse(false);
      index === strings.length - 1 ? setIndex(0) : setIndex(index + 1);
      return;
    }

    const timeout = setTimeout(
      () => {
        setSubIndex(subIndex + (reverse ? -1 : 1));
      },
      reverse ? 50 : subIndex === strings[index].length ? 1000 : 100
    );

    return () => clearTimeout(timeout);
  }, [strings, index, subIndex, reverse]);

  return (
    <>
      <span>{prefix}</span>
      <span className={className}>{strings[index].substring(0, subIndex)}</span>
      <span>{suffix}</span>
    </>
  );
}

export default Typewriter;
