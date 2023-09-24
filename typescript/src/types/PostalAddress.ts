// Generated file; do not edit. See `../rust/schema-gen` crate.

import { ContactPoint } from './ContactPoint';

// A physical mailing address.
export class PostalAddress extends ContactPoint {
  type = "PostalAddress";

  // The street address.
  streetAddress?: string;

  // The post office box number.
  postOfficeBoxNumber?: string;

  // The locality in which the street address is, and which is in the region.
  addressLocality?: string;

  // The region in which the locality is, and which is in the country.
  addressRegion?: string;

  // The postal code.
  postalCode?: string;

  // The country.
  addressCountry?: string;

  constructor(options?: PostalAddress) {
    super()
    if (options) Object.assign(this, options)
    
  }

  static from(other: PostalAddress): PostalAddress {
    return new PostalAddress(other)
  }
}
