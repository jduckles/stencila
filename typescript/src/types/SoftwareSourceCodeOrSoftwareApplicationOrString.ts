// Generated file; do not edit. See `../rust/schema-gen` crate.
            
import { SoftwareApplication } from './SoftwareApplication'
import { SoftwareSourceCode } from './SoftwareSourceCode'

// `SoftwareSourceCode` or `SoftwareApplication` or `string`
export type SoftwareSourceCodeOrSoftwareApplicationOrString =
  SoftwareSourceCode |
  SoftwareApplication |
  string;

export function softwareSourceCodeOrSoftwareApplicationOrString(other: SoftwareSourceCodeOrSoftwareApplicationOrString): SoftwareSourceCodeOrSoftwareApplicationOrString {
  if (other == null || typeof other !== "object" || Array.isArray(other) || typeof other.type === "undefined") {
    return other as SoftwareSourceCodeOrSoftwareApplicationOrString;
  }
  switch(other.type) {
    case "SoftwareSourceCode": return SoftwareSourceCode.from(other as SoftwareSourceCode);
    case "SoftwareApplication": return SoftwareApplication.from(other as SoftwareApplication);
    default: throw new Error(`Unexpected type for SoftwareSourceCodeOrSoftwareApplicationOrString: ${other.type}`)
  }
}
