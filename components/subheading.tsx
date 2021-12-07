import { useState, useEffect } from "react";

function Subheading({
  prefix,
  suffix,
  words,
}: {
  prefix?: string;
  suffix?: string;
  words: string[];
}) {
  const [index, setIndex] = useState(0);
  const [subIndex, setSubIndex] = useState(0);
  const [reverse, setReverse] = useState(false);

  useEffect(() => {
    if (subIndex === words[index].length + 1 && !reverse) {
      setReverse(true);
      return;
    }

    if (subIndex === 0 && reverse) {
      setReverse(false);
      index === words.length - 1 ? setIndex(0) : setIndex(index + 1);
      return;
    }

    const timeout = setTimeout(
      () => {
        setSubIndex(subIndex + (reverse ? -1 : 1));
      },
      reverse ? 50 : subIndex === words[index].length ? 1000 : 100
    );

    return () => clearTimeout(timeout);
  }, [words, subIndex, index, reverse]);

  return (
    <h2 className="text-xl font-semibold">
      <span>{prefix}</span>
      <span className="text-green-500">
        {words[index].substring(0, subIndex)}
      </span>
      <span>{suffix}</span>
    </h2>
  );
}

export default Subheading;
