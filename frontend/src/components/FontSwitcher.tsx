import { useCallback, useRef, type ChangeEvent } from "react";
import FontReader from "../font-reader";
import type { FontOverride } from "./App";

type FontSwitcherProps = {
  font: FontReader;
  setFontOverride: React.Dispatch<FontOverride>;
};
const FontSwitcher = ({ font, setFontOverride }: FontSwitcherProps) => {
  const inputRef = useRef<HTMLInputElement>(null);

  const handleClick = useCallback(() => {
    inputRef.current!.click();
  }, [inputRef]);

  const handleChange = useCallback(async (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) {
      return;
    }

    const buffer = await file.arrayBuffer();
    const font = FontReader.fromBuffer(buffer);
    const data = await new Promise<string>((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = () => resolve(reader.result as string);
      reader.onerror = reject;
      reader.readAsDataURL(file);
    });

    setFontOverride({
      font,
      data,
    });
  }, [setFontOverride]);

  return (
    <div>
      <span className="cursor-pointer font-foundry text-light" onClick={handleClick}>
        {font.name}
      </span>
      <input className="hidden" type="file" accept=".ttf" ref={inputRef} onChange={handleChange} />
    </div>
  );
};

export default FontSwitcher;
