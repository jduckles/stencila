// Generated file; do not edit. See `../rust/schema-gen` crate.

import { Entity } from './Entity';
import { Parameter } from './Parameter';
import { Validator } from './Validator';

// A function with a name, which might take Parameters and return a value of a certain type.
export class Function extends Entity {
  type = "Function";

  // The name of the function.
  name: string;

  // The parameters of the function.
  parameters: Parameter[];

  // The return type of the function.
  returns?: Validator;

  constructor(name: string, parameters: Parameter[], options?: Function) {
    super()
    if (options) Object.assign(this, options)
    this.name = name;
    this.parameters = parameters;
  }

  static from(other: Function): Function {
    return new Function(other.name!, other.parameters!, other)
  }
}
