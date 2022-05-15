mod permutation_iter;
mod rule_map;
mod rule_parser;
mod truth_table;

pub use permutation_iter::*;
pub use rule_map::*;
pub use rule_parser::*;
pub use truth_table::*;

use std::{borrow::Borrow, char};

pub fn is_identifier(c: impl Borrow<char>) -> bool {
    ('A'..='Z').contains(c.borrow())
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn all() {
        for c in '\0'..=char::MAX {
            assert_eq!(is_identifier(&c), ('A'..='Z').contains(&c));
        }
    }
}
