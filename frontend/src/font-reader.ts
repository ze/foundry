import { parse, type Font } from "opentype.js";

export default class FontReader {
  private static readonly LANGUAGE = "en";

  private readonly font: Font;
  private readonly _characters: string[];

  constructor(font: Font) {
    this.font = font;

    const characters: string[] = [];
    for (let i = 0; i < this.font.glyphs.length; i++) {
      const glyph = this.font.glyphs.get(i);
      if (glyph.unicode) {
        characters.push(String.fromCharCode(glyph.unicode));
      }
    }
    this._characters = characters;
  }

  static fromBuffer(arrayBuffer: ArrayBuffer): FontReader {
    const font = parse(arrayBuffer);
    return new FontReader(font);
  }

  get characters(): string[] {
    return this._characters;
  }

  get numbers(): string[] {
    return this.characters.filter(FontReader.isNumber);
  }

  get alphaLower(): string[] {
    return this.characters.filter(FontReader.isAlphaLower);
  }

  get alphaUpper(): string[] {
    return this.characters.filter(FontReader.isAlphaUpper);
  }

  get alpha(): string[] {
    return this.characters.filter(FontReader.isAlpha);
  }

  get alphanumeric(): string[] {
    return this.characters.filter((c) => FontReader.isAlpha(c) || FontReader.isNumber(c));
  }

  get symbols(): string[] {
    return this.characters.filter((c) => c in [`!"#$%&'()*+,-./`]);
  }

  private static isAlphaLower(c: string): boolean {
    return c >= "a" && c <= "z";
  }

  private static isAlphaUpper(c: string): boolean {
    return c >= "A" && c <= "Z";
  }

  private static isAlpha(c: string): boolean {
    return FontReader.isAlphaLower(c) || FontReader.isAlphaUpper(c);
  }

  private static isNumber(c: string): boolean {
    return c >= "0" && c <= "9";
  }

  get name(): string {
    return this.font.names.fullName[FontReader.LANGUAGE];
  }
}
