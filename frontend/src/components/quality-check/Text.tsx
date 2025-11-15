import { useCallback, useState, type ChangeEvent, type ReactNode } from "react";

type WrappedGroupProps = {
  children: ReactNode;
};
export const Group = ({ children }: WrappedGroupProps) => (
  <div className="row w-full max-w-[90vw] [&>:not(:last-child)]:mr-2">{children}</div>
);

type WrappedProps = {
  label: string;
  left: string;
  strings: string[];
  right: string;
  columns: number;
  reversible?: boolean;
  size?: TextBlockSize;
};
export const Wrapped = ({ label, left, strings, right, columns, reversible, size = "regular" }: WrappedProps) => {
  const [isReversed, setReversed] = useState(false);
  return (
    <TextBlock
      label={label}
      block={wrap(left, strings, right, columns)}
      reversible={reversible ?? false}
      reversed={isReversed}
      setReversed={setReversed}
      size={size}
    />
  );
};

export type PermutationsProps = {
  label: string;
  left: string[];
  right?: string[];
  reversible?: boolean;
};
export const Permutations = ({ label, left, right = left, reversible = false }: PermutationsProps) => {
  const [isReversed, setReversed] = useState(false);

  return (
    <TextBlock
      label={label}
      block={permutations(left, right)}
      reversible={reversible}
      reversed={isReversed}
      setReversed={setReversed}
    />
  );
};

type TextBlockSize = "small" | "regular";
type TextBlockProps = {
  label: string;
  block: string[][];
  reversible: boolean;
  reversed: boolean;
  setReversed: React.Dispatch<boolean>;
  size?: TextBlockSize;
};
const TextBlock = ({ label, block, reversible, reversed, setReversed, size }: TextBlockProps) => {
  const handleChange = useCallback(
    (event: ChangeEvent<HTMLInputElement>) => {
      setReversed(event.target.checked);
    },
    [setReversed]
  );

  const flex = size === "small" ? "flex-1" : "flex-2";
  return (
    <div className={`w-full max-w-[90vw] min-w-0 rounded-sm border border-orange bg-dark ${flex}`}>
      <div className="mb-0.5 row-between">
        <div className="w-fit rounded-br-sm border-r border-b border-orange px-3">
          <span>{label}</span>
        </div>
        {reversible && (
          <div className="w-fit rounded-bl-sm border-b border-l border-orange px-3">
            <span className="mr-1.5">Reversed:</span>
            <input type="checkbox" onChange={handleChange} />
          </div>
        )}
      </div>
      <div className="overflow-x-scroll">
        <div className="min-w-fit px-3 pb-2 text-3xl">
          {block.map((row, i) => (
            <TextRow row={row} key={i} reversed={reversed} />
          ))}
        </div>
      </div>
    </div>
  );
};

type TextRowProps = {
  row: string[];
  reversed: boolean;
};
const TextRow = ({ row, reversed }: TextRowProps) => {
  return (
    <div className="row-between min-w-fit">
      {row.map((chunk, i) => (
        <TextChunk chunk={chunk} key={i} index={i} reversed={reversed} />
      ))}
    </div>
  );
};

type TextChunkProps = {
  chunk: string;
  index: number;
  reversed: boolean;
};
const TextChunk = ({ chunk, index, reversed }: TextChunkProps) => {
  const text = reversed ? chunk.split("").reverse().join("") : chunk;
  return <span className={"whitespace-nowrap" + (index == 0 ? "" : " ml-1.5")}>{text}</span>;
};

function permutations(left: string[], right: string[]): string[][] {
  const results: string[][] = [];
  for (const l of left) {
    const result: string[] = [];
    for (const r of right) {
      result.push(l + r);
    }
    results.push(result);
  }
  return results;
}

function wrap(left: string, strings: string[], right: string, groupSize: number): string[][] {
  const wrapped = strings.map((s) => left + s + right);
  const result: string[][] = [];

  for (let i = 0; i < wrapped.length; i += groupSize) {
    result.push(wrapped.slice(i, i + groupSize));
  }

  return result;
}
