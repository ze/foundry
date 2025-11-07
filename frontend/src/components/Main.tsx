import type FontReader from "../font-reader";
import InputArea from "./InputArea";
import QualityCheck from "./quality-check/QualityCheck";

type MainProps = {
  font: FontReader;
};
const Main = ({ font }: MainProps) => (
  <div className="column-center">
    <div className="mt-2.5">
      <InputArea />
    </div>
    <div className="mt-2.5">
      <QualityCheck font={font} />
    </div>
  </div>
);

export default Main;
