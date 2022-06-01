use super::is_identifier;

use std::{borrow::Borrow, collections::HashSet};

// PermutationIter is an iterator that iterates over all permutations of a rule input string
// The order in which the permutations are generated is always following the same pattern, example:
// `A => B` produces the following permutations:
// `0 => 0`
// `0 => 1`
// `1 => 0`
// `1 => 1`
pub struct PermutationIter {
    formula: String,
    pub variables: Vec<char>,
    size: usize,
}

impl PermutationIter {
    pub fn new<T>(formula: T) -> PermutationIter
    where
        T: Borrow<str>,
    {
        let formula = formula.borrow().to_owned();
        let mut set = HashSet::new();
        let mut variables = formula
            .chars()
            .filter(|c| is_identifier(c) && set.insert(c.to_owned()))
            .collect::<Vec<char>>();
        variables.sort_unstable();
        PermutationIter {
            formula,
            variables,
            size: 0,
        }
    }
}

impl Iterator for PermutationIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 1 << self.variables.len() {
            None
        } else {
            let mut permutation = self.formula.clone();
            for (i, c) in self.variables.iter().enumerate() {
                permutation = permutation.replace(
                    &c.to_string(),
                    if self.size & (1 << (self.variables.len() - 1 - i)) == 0 {
                        "0"
                    } else {
                        "1"
                    },
                );
            }
            self.size += 1;
            Some(permutation)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn empty() {
        let mut iter = PermutationIter::new("");
        assert_eq!(Some("".to_string()), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn identifiers() {
        let mut iter = PermutationIter::new("! 1 0 , . a z A Z");
        assert_eq!(Some("! 1 0 , . a z 0 0".to_string()), iter.next());
        assert_eq!(Some("! 1 0 , . a z 0 1".to_string()), iter.next());
        assert_eq!(Some("! 1 0 , . a z 1 0".to_string()), iter.next());
        assert_eq!(Some("! 1 0 , . a z 1 1".to_string()), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn order() {
        let mut iter = PermutationIter::new("A B C");
        assert_eq!(Some("0 0 0".to_string()), iter.next());
        assert_eq!(Some("0 0 1".to_string()), iter.next());
        assert_eq!(Some("0 1 0".to_string()), iter.next());
        assert_eq!(Some("0 1 1".to_string()), iter.next());
        assert_eq!(Some("1 0 0".to_string()), iter.next());
        assert_eq!(Some("1 0 1".to_string()), iter.next());
        assert_eq!(Some("1 1 0".to_string()), iter.next());
        assert_eq!(Some("1 1 1".to_string()), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn duplicate_identifiers() {
        let mut iter = PermutationIter::new("A A B B");
        assert_eq!(Some("0 0 0 0".to_string()), iter.next());
        assert_eq!(Some("0 0 1 1".to_string()), iter.next());
        assert_eq!(Some("1 1 0 0".to_string()), iter.next());
        assert_eq!(Some("1 1 1 1".to_string()), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn with_rule_symbols() {
        let mut iter = PermutationIter::new("A + B <=> C");
        assert_eq!(Some("0 + 0 <=> 0".to_string()), iter.next());
        assert_eq!(Some("0 + 0 <=> 1".to_string()), iter.next());
        assert_eq!(Some("0 + 1 <=> 0".to_string()), iter.next());
        assert_eq!(Some("0 + 1 <=> 1".to_string()), iter.next());
        assert_eq!(Some("1 + 0 <=> 0".to_string()), iter.next());
        assert_eq!(Some("1 + 0 <=> 1".to_string()), iter.next());
        assert_eq!(Some("1 + 1 <=> 0".to_string()), iter.next());
        assert_eq!(Some("1 + 1 <=> 1".to_string()), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn respect_white_space() {
        let mut iter = PermutationIter::new("\t\n\r A");
        assert_eq!(Some("\t\n\r 0".to_string()), iter.next());
        assert_eq!(Some("\t\n\r 1".to_string()), iter.next());
        assert_eq!(None, iter.next());
    }
}
