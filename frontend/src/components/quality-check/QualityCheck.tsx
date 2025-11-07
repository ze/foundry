import type FontReader from "../../font-reader";
import { Permutations, Wrapped, WrappedGroup } from "./Text";

type QualityCheckProps = {
  font: FontReader;
};
const QualityCheck = ({ font }: QualityCheckProps) => {
  return (
    <div className="column-center font-foundry text-light [&>:not(:last-child)]:mb-2">
      <Permutations label="Lowercase Alpha" left={font.alphaLower} />
      <Permutations label="Uppercase Alpha" left={font.alphaUpper} />
      <Permutations label="Alpha" left={font.alphaLower} right={font.alphaUpper} reversible />
      <WrappedGroup>
        <Wrapped label="Parenthesis - numbers" left="(" strings={font.numbers} right=")" columns={5} size="small" />
        <Wrapped label="Parenthesis - lower" left="(" strings={font.alphaLower} right=")" columns={13} />
        <Wrapped label="Parenthesis - upper" left="(" strings={font.alphaUpper} right=")" columns={13} />
      </WrappedGroup>
      <WrappedGroup>
        <Wrapped label="Braces - numbers" left="{" strings={font.numbers} right="}" columns={5} size="small" />
        <Wrapped label="Braces - lower" left="{" strings={font.alphaLower} right="}" columns={13} />
        <Wrapped label="Braces - upper" left="{" strings={font.alphaUpper} right="}" columns={13} />
      </WrappedGroup>
      <WrappedGroup>
        <Wrapped label="Brackets - numbers" left="[" strings={font.numbers} right="]" columns={5} size="small" />
        <Wrapped label="Brackets - lower" left="[" strings={font.alphaLower} right="]" columns={13} />
        <Wrapped label="Brackets - upper" left="[" strings={font.alphaUpper} right="]" columns={13} />
      </WrappedGroup>
    </div>
  );
};

export default QualityCheck;
