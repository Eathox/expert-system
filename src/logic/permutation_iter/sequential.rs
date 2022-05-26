use crate::is_identifier;

use std::{borrow::Borrow, collections::HashMap};

pub struct SequentialPermutationIter {
    pub variables: Vec<char>,
    formula: String,
    pos_map: HashMap<char, Vec<usize>>,
    permutation: usize,
}

impl SequentialPermutationIter {
    pub fn new<T>(formula: T) -> SequentialPermutationIter
    where
        T: Borrow<str>,
    {
        let formula = formula.borrow().to_owned();
        let mut pos_map = HashMap::new();
        for (i, c) in formula.chars().enumerate() {
            if is_identifier(c) {
                pos_map.entry(c).or_insert_with(Vec::new).push(i);
            }
        }
        let mut variables: Vec<char> = pos_map.keys().cloned().collect();
        variables.sort_unstable();
        SequentialPermutationIter {
            variables,
            formula,
            pos_map,
            permutation: 0,
        }
    }
}

impl Iterator for SequentialPermutationIter {
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
