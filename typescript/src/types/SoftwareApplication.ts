// Generated file; do not edit. See `../rust/schema-gen` crate.

import { CreativeWork } from './CreativeWork';

// A software application.
export class SoftwareApplication extends CreativeWork {
  type = "SoftwareApplication";

  // Requirements for application, including shared libraries that
  // are not included in the application distribution.
  softwareRequirements?: SoftwareApplication[];

  // Version of the software.
  softwareVersion?: string;

  constructor(name: string, options?: SoftwareApplication) {
    super()
    if (options) Object.assign(this, options)
    this.name = name;
  }

  static from(other: SoftwareApplication): SoftwareApplication {
    return new SoftwareApplication(other.name!, other)
  }
}
