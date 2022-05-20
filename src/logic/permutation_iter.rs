use super::is_identifier;

use std::{borrow::Borrow, collections::HashMap};

// PermutationIter is an iterator that iterates over all permutations of a rule input string
// The order in which the permutations are generated is always following the same pattern, example:
// `A => B` produces the following permutations:
// `0 => 0`
// `0 => 1`
// `1 => 0`
// `1 => 1`
pub struct PermutationIter {
    pub variables: Vec<char>,
    formula: String,
    pos_map: HashMap<char, Vec<usize>>,
    permutation: usize,
}

impl<T> From<T> for PermutationIter
where
    T: Borrow<str>,
{
    fn from(formula: T) -> PermutationIter {
        let formula = formula.borrow().to_owned();
        let mut pos_map = HashMap::new();
        for (i, c) in formula.chars().enumerate() {
            if is_identifier(c) {
                pos_map.entry(c).or_insert_with(Vec::new).push(i);
            }
        }
        let mut variables: Vec<char> = pos_map.keys().cloned().collect();
        variables.sort_unstable();
        PermutationIter {
            variables,
            formula,
            pos_map,
            permutation: 0,
        }
    }
}

impl Iterator for PermutationIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.permutation == 1 << self.variables.len() {
            None
        } else {
            let mut permutation = self.formula.clone();
            for (i, c) in self.variables.iter().enumerate() {
                for &pos in self.pos_map[c].iter() {
                    permutation.replace_range(pos..(pos + 1), {
                        if self.permutation & (1 << (self.variables.len() - 1 - i)) == 0 {
                            "0"
                        } else {
                            "1"
                        }
                    })
                }
            }
            self.permutation += 1;
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
        let mut iter = PermutationIter::from("");
        assert_eq!(Some("".to_string()), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn identifiers() {
        let mut iter = PermutationIter::from("! 1 0 , . a z A Z");
        assert_eq!(Some("! 1 0 , . a z 0 0".to_string()), iter.next());
        assert_eq!(Some("! 1 0 , . a z 0 1".to_string()), iter.next());
        assert_eq!(Some("! 1 0 , . a z 1 0".to_string()), iter.next());
        assert_eq!(Some("! 1 0 , . a z 1 1".to_string()), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn order() {
        let mut iter = PermutationIter::from("A B C");
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
        let mut iter = PermutationIter::from("A A B B");
        assert_eq!(Some("0 0 0 0".to_string()), iter.next());
        assert_eq!(Some("0 0 1 1".to_string()), iter.next());
        assert_eq!(Some("1 1 0 0".to_string()), iter.next());
        assert_eq!(Some("1 1 1 1".to_string()), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn with_rule_symbols() {
        let mut iter = PermutationIter::from("A + B <=> C");
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
        let mut iter = PermutationIter::from("\t\n\r A");
        assert_eq!(Some("\t\n\r 0".to_string()), iter.next());
        assert_eq!(Some("\t\n\r 1".to_string()), iter.next());
        assert_eq!(None, iter.next());
    }
}
