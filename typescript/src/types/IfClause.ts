// Generated file; do not edit. See `../rust/schema-gen` crate.

import { Block } from "./Block.js";
import { CodeExecutable } from "./CodeExecutable.js";
import { Cord } from "./Cord.js";

/**
 * A clause within a `If` node
 */
export class IfClause extends CodeExecutable {
  type = "IfClause";

  /**
   * Whether this clause is the active clause in the parent `If` node
   */
  isActive?: boolean;

  /**
   * The content to render if the result is truthy
   */
  content: Block[];

  constructor(code: Cord, programmingLanguage: string, content: Block[], options?: Partial<IfClause>) {
    super(code, programmingLanguage);
    if (options) Object.assign(this, options);
    this.code = code;
    this.programmingLanguage = programmingLanguage;
    this.content = content;
  }
}

/**
* Create a new `IfClause`
*/
export function ifClause(code: Cord, programmingLanguage: string, content: Block[], options?: Partial<IfClause>): IfClause {
  return new IfClause(code, programmingLanguage, content, options);
}
