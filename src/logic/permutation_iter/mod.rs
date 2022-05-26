mod parallel;
mod sequential;

use parallel::*;
use sequential::*;

use std::borrow::Borrow;

const MAX_SEQUENTIAL_VARIABLES: usize = 18;
const PARALLEL_BUFF_SIZE: usize = 10000;

enum SmartPermutationIter {
    Sequential(SequentialPermutationIter),
    Parallel(ParallelPermutationIter),
}

impl SmartPermutationIter {
    fn new(sequential_iter: SequentialPermutationIter) -> SmartPermutationIter {
        match sequential_iter.variables.len() {
            0..=MAX_SEQUENTIAL_VARIABLES => SmartPermutationIter::Sequential(sequential_iter),
            _ => SmartPermutationIter::Parallel(ParallelPermutationIter::new(
                sequential_iter,
                PARALLEL_BUFF_SIZE,
            )),
        }
    }
}

// PermutationIter is an iterator that iterates over all permutations of a rule input string
// The order in which the permutations are generated is always following the same pattern, example:
// `A => B` produces the following permutations:
// `0 => 0`
// `0 => 1`
// `1 => 0`
// `1 => 1`
pub struct PermutationIter {
    pub variables: Vec<char>,
    iter: SmartPermutationIter,
}

impl<T> From<T> for PermutationIter
where
    T: Borrow<str>,
{
    fn from(formula: T) -> PermutationIter {
        let sequential_iter = SequentialPermutationIter::new(formula);
        let variables = sequential_iter.variables.to_owned();
        PermutationIter {
            variables,
            iter: SmartPermutationIter::new(sequential_iter),
        }
    }
}

impl Iterator for PermutationIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter {
            SmartPermutationIter::Sequential(ref mut iter) => iter.next(),
            SmartPermutationIter::Parallel(ref mut iter) => iter.next(),
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
