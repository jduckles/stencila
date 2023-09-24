// Generated file; do not edit. See `../rust/schema-gen` crate.

import { Cord } from './Cord';
import { Entity } from './Entity';
import { ExecutionDigest } from './ExecutionDigest';

// An abstract base class for a document node that has styling applied to it and/or its content
export class Styled extends Entity {
  type = "Styled";

  // The code of the equation in the `styleLanguage`.
  code: Cord;

  // The language used for the style specification e.g. css, tailwind, classes.
  styleLanguage?: string;

  // A digest of the `code` and `styleLanguage`.
  compileDigest?: ExecutionDigest;

  // Errors that occurred when transpiling the `code`.
  errors?: string[];

  // A Cascading Style Sheet (CSS) transpiled from the `code` property.
  css?: string;

  // A list of class names associated with the node
  classes?: string[];

  constructor(code: Cord, options?: Styled) {
    super()
    if (options) Object.assign(this, options)
    this.code = code;
  }

  static from(other: Styled): Styled {
    return new Styled(other.code!, other)
  }
}
