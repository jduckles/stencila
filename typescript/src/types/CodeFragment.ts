// Generated file; do not edit. See `../rust/schema-gen` crate.

import { CodeStatic } from "./CodeStatic.js";
import { Cord } from "./Cord.js";

// Inline code.
export class CodeFragment extends CodeStatic {
  type = "CodeFragment";

  constructor(code: Cord, options?: CodeFragment) {
    super(code);
    if (options) Object.assign(this, options);
    this.code = code;
  }

  static from(other: CodeFragment): CodeFragment {
    return new CodeFragment(other.code!, other);
  }
}
