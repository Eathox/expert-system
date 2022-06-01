use std::collections::HashMap;

pub struct SequentialPermutationIter {
    pub variables: Vec<char>,
    formula: String,
    pos_map: HashMap<char, Vec<usize>>,
    permutation: usize,
    end: usize,
}

impl SequentialPermutationIter {
    pub fn new(
        formula: String,
        variables: Vec<char>,
        pos_map: HashMap<char, Vec<usize>>,
        start: usize,
        end: usize,
    ) -> SequentialPermutationIter {
        SequentialPermutationIter {
            variables,
            formula,
            pos_map,
            permutation: start,
            end,
        }
    }
}

impl Iterator for SequentialPermutationIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.permutation == self.end {
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
