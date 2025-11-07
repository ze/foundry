import { useCallback, useState, type ChangeEvent } from "react";

const InputArea = () => {
  const [fontSize, setFontSize] = useState<number>(32);
  const [lineHeight, setLineHeight] = useState<number>(1.2);

  const handleFontSize = useCallback(
    (event: ChangeEvent<HTMLInputElement>) => setFontSize(event.target.valueAsNumber),
    []
  );
  const handleLineHeight = useCallback(
    (event: ChangeEvent<HTMLInputElement>) => setLineHeight(event.target.valueAsNumber),
    []
  );

  return (
    <div className="column w-fit">
      <div className="mb-2.25 row w-full justify-between px-1.75">
        <div className="column grow-6">
          <label htmlFor="font-size" className="font-foundry text-sm text-light">
            Font size: {fontSize}px
          </label>
          <input
            type="range"
            name="font-size"
            className="mt-1.5 accent-orange"
            min={6}
            max={72}
            step={2}
            value={fontSize}
            onChange={handleFontSize}
          />
        </div>
        <div className="ml-4 column grow">
          <label htmlFor="line-height" className="font-foundry text-sm text-light">
            Line height: {lineHeight}
          </label>
          <input
            type="range"
            name="line-height"
            className="mt-1.5 accent-orange"
            min={0.1}
            max={2}
            step={0.1}
            value={lineHeight}
            onChange={handleLineHeight}
          />
        </div>
      </div>
      <textarea
        className="mb-0.5 h-75 w-3xl resize-none rounded-sm border border-orange bg-dark px-2.5 py-1 font-foundry text-light accent-orange"
        style={{ fontSize, lineHeight }}
        spellCheck="false"
        autoCapitalize="off"
        autoComplete="off"
        autoCorrect="off"
      ></textarea>
    </div>
  );
};

export default InputArea;
