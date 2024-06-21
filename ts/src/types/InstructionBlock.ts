// Generated file; do not edit. See https://github.com/stencila/stencila/tree/main/rust/schema-gen

import { Block } from "./Block.js";
import { Instruction } from "./Instruction.js";
import { InstructionMessage } from "./InstructionMessage.js";
import { InstructionType } from "./InstructionType.js";
import { SuggestionBlock } from "./SuggestionBlock.js";

/**
 * An instruction to edit some block content.
 */
export class InstructionBlock extends Instruction {
  // @ts-expect-error 'not assignable to the same property in base type'
  type: "InstructionBlock";

  /**
   * The content to which the instruction applies.
   */
  content?: Block[];

  /**
   * Suggestions for the instruction
   */
  suggestions?: SuggestionBlock[];

  constructor(instructionType: InstructionType, messages: InstructionMessage[], options?: Partial<InstructionBlock>) {
    super(instructionType, messages);
    this.type = "InstructionBlock";
    if (options) Object.assign(this, options);
    this.instructionType = instructionType;
    this.messages = messages;
  }
}

/**
* Create a new `InstructionBlock`
*/
export function instructionBlock(instructionType: InstructionType, messages: InstructionMessage[], options?: Partial<InstructionBlock>): InstructionBlock {
  return new InstructionBlock(instructionType, messages, options);
}
