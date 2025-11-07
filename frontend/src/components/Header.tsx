import type React from "react";
import type FontReader from "../font-reader";
import type { FontOverride } from "./App";
import FontSwitcher from "./FontSwitcher";
import Spinner from "./Spinner";

type HeaderProps = {
  font: FontReader | undefined;
  setFontOverride: React.Dispatch<FontOverride>;
};
const Header = ({ font, setFontOverride }: HeaderProps) => (
  <nav className="sticky top-0 z-1 row-between border-b border-b-orange bg-dark px-2.5 py-0.75">
    <div>
      <span className="font-foundry font-bold text-orange">FOUNDRY</span>
    </div>
    {font ? <FontSwitcher font={font} setFontOverride={setFontOverride} /> : <Spinner />}
  </nav>
);

export default Header;
