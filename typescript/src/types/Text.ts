// Generated file; do not edit. See `../rust/schema-gen` crate.

import { Cord } from "./Cord.js";
import { Entity } from "./Entity.js";

// Textual content
export class Text extends Entity {
  type = "Text";

  // The value of the text content
  value: Cord;

  constructor(value: Cord, options?: Text) {
    super();
    if (options) Object.assign(this, options);
    this.value = value;
  }

  static from(other: Text): Text {
    return new Text(other.value!, other);
  }
}
