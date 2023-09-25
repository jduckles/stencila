// Generated file; do not edit. See `../rust/schema-gen` crate.
            
import { PostalAddress } from "./PostalAddress.js";

// `PostalAddress` or `string`
export type PostalAddressOrString =
  PostalAddress |
  string;

export function postalAddressOrStringFrom(other: PostalAddressOrString): PostalAddressOrString {
  if (other == null || typeof other !== "object" || Array.isArray(other) || typeof other.type === "undefined") {
    return other as PostalAddressOrString;
  }
  switch(other.type) {
    case "PostalAddress": return PostalAddress.from(other as PostalAddress);
    default: throw new Error(`Unexpected type for PostalAddressOrString: ${other.type}`);
  }
}
