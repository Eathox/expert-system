use crate::is_identifier;

mod parallel;
mod sequential;

use parallel::*;
use sequential::*;

use std::{borrow::Borrow, collections::HashMap};

const MAX_SEQUENTIAL_VARIABLES: usize = 15;

// PermutationIter is an iterator that iterates over all permutations of a rule input string
// The order in which the permutations are generated is always following the same pattern, example:
// `A => B` produces the following permutations:
// `0 => 0`
// `0 => 1`
// `1 => 0`
// `1 => 1`
pub enum PermutationIter {
    Sequential(SequentialPermutationIter),
    Parallel(ParallelPermutationIter),
}

fn calc_thread_count(variable_count: usize) -> usize {
    if variable_count <= MAX_SEQUENTIAL_VARIABLES {
        0
    } else {
        (variable_count - MAX_SEQUENTIAL_VARIABLES) / 2
    }
}

impl PermutationIter {
    pub fn new<T>(formula: T) -> PermutationIter
    where
        T: Borrow<str>,
    {
        let formula = formula.borrow().to_string();

        let mut pos_map = HashMap::new();
        for (i, c) in formula.chars().enumerate() {
            if is_identifier(c) {
                pos_map.entry(c).or_insert_with(Vec::new).push(i);
            }
        }
        let mut variables: Vec<char> = pos_map.keys().cloned().collect();
        variables.sort_unstable();

        let thread_count = calc_thread_count(variables.len());
        match thread_count {
            0 => {
                let end = 1 << variables.len();
                PermutationIter::Sequential(SequentialPermutationIter::new(
                    formula, variables, pos_map, 0, end,
                ))
            }
            _ => PermutationIter::Parallel(ParallelPermutationIter::new(
                formula,
                variables,
                pos_map,
                thread_count,
            )),
        }
    }

    pub fn variables(&self) -> &Vec<char> {
        match self {
            PermutationIter::Sequential(iter) => &iter.variables,
            PermutationIter::Parallel(iter) => &iter.variables,
        }
    }
}

impl Iterator for PermutationIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            PermutationIter::Sequential(ref mut iter) => iter.next(),
            PermutationIter::Parallel(ref mut iter) => iter.next(),
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
