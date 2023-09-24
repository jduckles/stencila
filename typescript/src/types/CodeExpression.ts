// Generated file; do not edit. See `../rust/schema-gen` crate.

import { CodeExecutable } from './CodeExecutable';
import { Cord } from './Cord';
import { Node } from './Node';

// An executable programming code expression.
export class CodeExpression extends CodeExecutable {
  type = "CodeExpression";

  // The value of the expression when it was last evaluated.
  output?: Node;

  constructor(code: Cord, programmingLanguage: string, options?: CodeExpression) {
    super(code, programmingLanguage)
    if (options) Object.assign(this, options)
    this.code = code;
    this.programmingLanguage = programmingLanguage;
  }

  static from(other: CodeExpression): CodeExpression {
    return new CodeExpression(other.code!, other.programmingLanguage!, other)
  }
}
