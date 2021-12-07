import { useState, useEffect } from "react";

function Typewriter({
  prefix,
  prefixVowel,
  strings,
  suffix,
  className,
}: {
  prefix?: string;
  prefixVowel?: string;
  strings: string[];
  suffix?: string;
  className?: string;
}): JSX.Element {
  const [index, setIndex] = useState<number>(0);
  const [subIndex, setSubIndex] = useState<number>(0);
  const [reverse, setReverse] = useState<boolean>(false);

  const [vowel, setVowel] = useState<boolean>(false);

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

    setVowel(
      ["a", "e", "i", "o", "u"].includes(
        (strings[index].charAt(0) ?? "").toLowerCase()
      )
    );

    return () => clearTimeout(timeout);
  }, [strings, index, subIndex, reverse]);

  return (
    <>
      <span>{prefixVowel && vowel ? prefixVowel : prefix}</span>
      <span className={className}>{strings[index].substring(0, subIndex)}</span>
      <span>{suffix}</span>
    </>
  );
}

export default Typewriter;
