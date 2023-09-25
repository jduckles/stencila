// Generated file; do not edit. See `../rust/schema-gen` crate.

import { Block } from "./Block.js";
import { Cord } from "./Cord.js";
import { Styled } from "./Styled.js";

// Styled block content
export class Division extends Styled {
  type = "Division";

  // The content within the division
  content: Block[];

  constructor(code: Cord, content: Block[], options?: Division) {
    super(code);
    if (options) Object.assign(this, options);
    this.code = code;
    this.content = content;
  }

  static from(other: Division): Division {
    return new Division(other.code!, other.content!, other);
  }
}
