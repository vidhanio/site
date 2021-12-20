import { useEffect, useState } from "react";

type Props = {
  prefix?: string;
  prefixVowel?: string;
  suffix?: string;
  strings: string[];
  className?: string;
};

export default function Typewriter({
  prefix,
  prefixVowel,
  strings,
  suffix,
  className,
}: Props): JSX.Element {
  const [index, setIndex] = useState<number>(0);
  const [subIndex, setSubIndex] = useState<number>(0);
  const [isReversed, setIsReversed] = useState<boolean>(false);

  const [isVowel, setIsVowel] = useState<boolean>(false);

  useEffect(() => {
    if (subIndex === strings[index].length + 1 && !isReversed) {
      setIsReversed(true);
      return;
    }

    if (subIndex === 0 && isReversed) {
      setIsReversed(false);
      index === strings.length - 1 ? setIndex(0) : setIndex(index + 1);
      return;
    }

    const timeout = setTimeout(
      () => {
        setSubIndex(subIndex + (isReversed ? -1 : 1));
      },
      isReversed ? 50 : subIndex === strings[index].length ? 1000 : 100
    );

    setIsVowel(
      ["a", "e", "i", "o", "u"].includes(
        (strings[index].charAt(0) ?? "").toLowerCase()
      )
    );

    return () => clearTimeout(timeout);
  }, [strings, index, subIndex, isReversed]);

  return (
    <>
      <span>{prefixVowel && isVowel ? prefixVowel : prefix}</span>
      <span className={className}>{strings[index].substring(0, subIndex)}</span>
      <span>{suffix}</span>
    </>
  );
}
