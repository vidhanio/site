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
  const [wordIndex, setWordIndex] = useState<number>(0);
  const [index, setIndex] = useState<number>(0);
  const [reverse, setReverse] = useState<boolean>(false);

  useEffect(() => {
    if (index === words[wordIndex].length + 1 && !reverse) {
      setReverse(true);
      return;
    }

    if (index === 0 && reverse) {
      setReverse(false);
      wordIndex === words.length - 1
        ? setWordIndex(0)
        : setWordIndex(wordIndex + 1);
      return;
    }

    const timeout = setTimeout(
      () => {
        setIndex(index + (reverse ? -1 : 1));
      },
      reverse ? 50 : index === words[wordIndex].length ? 1000 : 100
    );

    return () => clearTimeout(timeout);
  }, [words, wordIndex, index, reverse]);

  return (
    <h2 className="text-xl font-semibold">
      <span>{prefix}</span>
      <span className="text-green-600 dark:text-green-500">
        {words[wordIndex].substring(0, index)}
      </span>
      <span>{suffix}</span>
    </h2>
  );
}

export default Subheading;
