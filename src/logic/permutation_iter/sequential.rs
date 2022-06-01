use std::collections::HashMap;

pub struct SequentialPermutationIter {
    pub variables: Vec<char>,
    pub formula: String,
    pub pos_map: HashMap<char, Vec<usize>>,
    pub permutation: usize,
    pub end: usize,
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
