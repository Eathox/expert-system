use crate::is_identifier;

mod parallel;
mod sequential;

use parallel::*;
use sequential::*;

use crossbeam::channel::{bounded, Receiver};
use std::{borrow::Borrow, collections::HashMap};

enum PermutationIterType {
    Sequential(SequentialPermutationIter),
    Parallel(Receiver<String>),
}

const MAX_SEQUENTIAL_VARIABLES: usize = 15;
const PARALLEL_THREAD_BUFF_SIZE: usize = 4000;

// PermutationIter is an iterator that iterates over all permutations of a rule input string
// The order in which the permutations are generated is always following the same pattern, example:
// `A => B` produces the following permutations:
// `0 => 0`
// `0 => 1`
// `1 => 0`
// `1 => 1`
pub struct PermutationIter {
    pub variables: Vec<char>,
    iter: PermutationIterType,
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
        PermutationIter {
            variables: variables.clone(),
            iter: match thread_count {
                0 => PermutationIter::new_sequential(formula, variables, pos_map),
                _ => PermutationIter::new_parallel(formula, variables, pos_map, thread_count),
            },
        }
    }

    fn new_sequential(
        formula: String,
        variables: Vec<char>,
        pos_map: HashMap<char, Vec<usize>>,
    ) -> PermutationIterType {
        let end = 1 << variables.len();
        let iter = SequentialPermutationIter {
            variables,
            formula,
            pos_map,
            permutation: 0,
            end,
        };
        PermutationIterType::Sequential(iter)
    }

    fn new_parallel(
        formula: String,
        variables: Vec<char>,
        pos_map: HashMap<char, Vec<usize>>,
        thread_count: usize,
    ) -> PermutationIterType {
        let total_end = 1 << variables.len();
        let mut chunked_iters = Vec::with_capacity(thread_count);
        match thread_count {
            0 => unreachable!(),
            1 => {
                chunked_iters.push(SequentialPermutationIter {
                    variables,
                    formula,
                    pos_map,
                    permutation: 0,
                    end: total_end,
                });
            }
            _ => {
                let step = total_end / thread_count;
                let mut start;
                let mut end;
                for i in 0..(thread_count) {
                    start = step * i;
                    end = start + step;
                    if i == (thread_count - 1) {
                        end = total_end;
                    }

                    chunked_iters.push(SequentialPermutationIter {
                        variables: variables.clone(),
                        formula: formula.clone(),
                        pos_map: pos_map.clone(),
                        permutation: start,
                        end,
                    });
                }
            }
        }

        let (sender, receiver) = bounded(PARALLEL_THREAD_BUFF_SIZE * thread_count);
        for iter in chunked_iters {
            ParallelPermutationIter::new(iter, sender.clone());
        }
        PermutationIterType::Parallel(receiver)
    }
}

impl Iterator for PermutationIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter {
            PermutationIterType::Sequential(ref mut iter) => iter.next(),
            PermutationIterType::Parallel(ref receiver) => receiver.recv().ok(),
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
