///! Universally unique identifiers.
///!
///! The identifiers generated by this module:
///!
///! - are URL safe
///! - contain information on the type of object that they
///!   are identifying
///! - have an extremely low probability of collision
///!
///! Generated identifiers have a fixed length of 32 characters made up
///! of two parts separated by a hyphen:
///!
///! - 2 characters in the range `[a-z]` that identifying the "family" of
///!   identifiers, usually the type of object the identifier is for
///!   e.g. `fi` = file, `re` = request
///!
///! - 20 characters in the range `[0-9A-Za-z]` that are randomly generated
///!
///! For project identifiers (those starting with 'pr') only lowercase
///! letters are used for compatibility with Docker image naming rules.
///!
///! The total size of the generated ids is 23 bytes which allows it to fit
///! inside a [`SmartString`](https://lib.rs/crates/smartstring) for better
///! performance that a plain old `String`.
///!
///! See
///!  - https://segment.com/blog/a-brief-history-of-the-uuid/
///!  - https://zelark.github.io/nano-id-cc/
///!  - https://gist.github.com/fnky/76f533366f75cf75802c8052b577e2a5
use eyre::{bail, Result};
use nanoid::nanoid;
use regex::Regex;
use strum::Display;

use crate::errors::Error;

/// The available families of identifiers
#[derive(Debug, Clone, Display)]
pub enum Family {
    #[strum(serialize = "no")]
    Node,

    #[strum(serialize = "do")]
    Document,

    #[strum(serialize = "fi")]
    File,

    #[strum(serialize = "sn")]
    Snapshot,

    #[strum(serialize = "pr")]
    Project,

    #[strum(serialize = "se")]
    Session,

    #[strum(serialize = "ke")]
    Kernel,

    #[strum(serialize = "cl")]
    Client,
}

/// The separator between the family and random parts of the identifier
///
/// A hyphen provides for better readability than a dot or colon when used
/// in pubsub topic strings and elsewhere.
const SEPARATOR: &str = "-";

/// The characters used in the random part of the identifier
const CHARACTERS: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

// Generate a universally unique identifier
pub fn generate(family: Family) -> String {
    let chars = match family {
        Family::Project => nanoid!(20, &CHARACTERS[..36]),
        _ => nanoid!(20, &CHARACTERS),
    };
    [&family.to_string(), SEPARATOR, &chars].concat()
}

// Test whether a string is an identifer for a particular family
pub fn matches(family: Family, id: &str) -> bool {
    let pattern = match family {
        Family::Project => "[0-9a-z]{20}",
        _ => "[0-9a-zA-Z]{20}",
    };
    let re = [&family.to_string(), SEPARATOR, pattern].concat();
    let re = Regex::new(&re).expect("Should be a valid regex");
    re.is_match(id)
}

// Assert that a string is an identifer for a particular family
pub fn assert(family: Family, id: &str) -> Result<String> {
    match matches(family.clone(), id) {
        true => Ok(id.to_string()),
        false => bail!(Error::InvalidUUID {
            family: family.to_string(),
            id: id.to_string()
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_id() {
        let id = generate(Family::Node);
        assert_eq!(id.len(), 23);
        assert!(matches(Family::Node, &id));
        assert(Family::Node, &id).unwrap();
    }

    #[test]
    fn test_project_id() {
        let id = generate(Family::Project);
        assert_eq!(id.len(), 23);
        assert!(matches(Family::Project, &id));
        assert(Family::Project, &id).unwrap();
    }
}
