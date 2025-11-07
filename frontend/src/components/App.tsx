import { useQuery } from "@tanstack/react-query";
import { useState } from "react";
import FontReader from "../font-reader";
import Header from "./Header";
import Main from "./Main";
import Spinner from "./Spinner";

export type FontOverride = {
  font: FontReader;
  data: string;
};

const App = () => {
  const { data, isPending } = useQuery({
    queryKey: ["font"],
    queryFn: async () => {
      const url = import.meta.env.DEV ? "https://rsms.me/inter/font-files/InterVariable.ttf" : "/api/font";
      const data = await fetch(url);
      const buffer = await data.arrayBuffer();
      return FontReader.fromBuffer(buffer);
    },
    staleTime: "static",
    retry: false,
  });

  const [fontOverride, setFontOverride] = useState<FontOverride>();
  const font = fontOverride?.font ?? data;

  return (
    <>
      {fontOverride && (
        <style>{`@font-face { font-family: "FoundryOverride"; src: url("${fontOverride.data}") format("truetype"); }`}</style>
      )}
      <Header font={font} setFontOverride={setFontOverride} />
      {isPending && (
        <div className="mt-2 row justify-center">
          <Spinner />
        </div>
      )}
      {font && <Main font={font} />}
    </>
  );
};

export default App;
