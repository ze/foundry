import type FontReader from "../../font-reader";
import { Group, Permutations, Wrapped } from "./Text";

type QualityCheckProps = {
  font: FontReader;
};
const QualityCheck = ({ font }: QualityCheckProps) => {
  return (
    <div className="column-center font-foundry text-light *:last:mb-10 [&>:not(:last-child)]:mb-2">
      <Permutations label="Lowercase Alpha" left={font.alphaLower} />
      <Permutations label="Uppercase Alpha" left={font.alphaUpper} />
      <Permutations label="Alpha" left={font.alphaLower} right={font.alphaUpper} reversible />
      <Group>
        <Permutations label="Numbers" left={font.numbers} />
        <Permutations label="Currency" left={font.numbers} right={font.currency} reversible />
      </Group>
      <CommonWrappedGroup font={font} label="Parenthesis" left="(" right=")" />
      <CommonWrappedGroup font={font} label="Braces" left="{" right="}" />
      <CommonWrappedGroup font={font} label="Brackets" left="[" right="]" />
      <CommonWrappedGroup font={font} label="Single quotes" left="'" />
      <CommonWrappedGroup font={font} label="Double quotes" left='"' />
      <CommonWrappedGroup font={font} label="Backtick" left="`" />
      <CommonWrappedGroup font={font} label="Question mark" left="¿" right="?" />
      <CommonWrappedGroup font={font} label="Exclamation mark" left="¡" right="!" />
      <CommonWrappedGroup font={font} label="Colon" left=":" />
      <CommonWrappedGroup font={font} label="Semi-colon" left=";" />
      <CommonWrappedGroup font={font} label="Period" left="." />
      <CommonWrappedGroup font={font} label="Forward slash" left="/" />
      <CommonWrappedGroup font={font} label="Backslash" left="\" />
      <CommonWrappedGroup font={font} label="Hyphen/Minus" left="-" />
      <CommonWrappedGroup font={font} label="Plus" left="+" />
      <CommonWrappedGroup font={font} label="Asterisk" left="*" />
      <CommonWrappedGroup font={font} label="Tilde" left="~" />
      <CommonWrappedGroup font={font} label="Octothorpe" left="#" />
      <CommonWrappedGroup font={font} label="Ampersand" left="&" />
      <CommonWrappedGroup font={font} label="At symbol" left="@" />
      <CommonWrappedGroup font={font} label="Caret" left="^" />
      <CommonWrappedGroup font={font} label="Underscore" left="_" />
      <CommonWrappedGroup font={font} label="Less than" left="<" />
      <CommonWrappedGroup font={font} label="Greater than" left=">" />
      <CommonWrappedGroup font={font} label="Equal to" left="=" />
      <Wrapped label="Everything" strings={font.nonWhitespace} left="" right="" columns={13} />
    </div>
  );
};

type CommonWrappedGroupProps = {
  font: FontReader;
  label: string;
  left: string;
  right?: string;
};
const CommonWrappedGroup = ({ font, label, left, right = left }: CommonWrappedGroupProps) => (
  <Group>
    <Wrapped label={`${label} - numbers`} left={left} strings={font.numbers} right={right} columns={5} size="small" />
    <Wrapped label={`${label} - lower`} left={left} strings={font.alphaLower} right={right} columns={13} />
    <Wrapped label={`${label} - upper`} left={left} strings={font.alphaUpper} right={right} columns={13} />
  </Group>
);

export default QualityCheck;
