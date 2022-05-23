use super::{evaluate_rule, PermutationIter};

use anyhow::{Context, Result};
use std::fmt;

// TruthTable struct holds the truth table data of an input rule.
// It can be constructed using a PermutationIter. Since the permutations generated by
// PermuationIter is always guaranteed to follow the same pattern, the order of the results
// implicitly holds the propositional data for each entry in the TruthTable. Example:
// `0 => 0` implies index 0b00, results[0]
// `0 => 1` implies index 0b01, results[1]
// `1 => 0` implies index 0b10, results[2]
// `1 => 1` implies index 0b11, results[3]
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct TruthTable {
    pub variables: Vec<char>,
    pub results: Vec<bool>,
}

impl TryFrom<PermutationIter> for TruthTable {
    type Error = anyhow::Error;

    fn try_from(mut permutation_iter: PermutationIter) -> Result<Self> {
        let results: Result<Vec<bool>> = permutation_iter
            .by_ref()
            .map(|permutation| evaluate_rule(&permutation))
            .collect();
        Ok(TruthTable {
            results: results.context("Failed to evaluate permutations")?,
            variables: permutation_iter.variables,
        })
    }
}

impl TryFrom<&str> for TruthTable {
    type Error = anyhow::Error;

    fn try_from(str: &str) -> Result<Self> {
        PermutationIter::from(str).try_into()
    }
}

impl fmt::Debug for TruthTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.variables.len();
        for v in &self.variables {
            write!(f, "| {} ", v)?;
        }
        writeln!(f, "| = |")?;
        writeln!(f, "{}|", "|---".repeat(len + 1))?;
        for (i, result) in self.results.iter().enumerate() {
            for b in 0..len {
                write!(
                    f,
                    "| {} ",
                    if i & (1 << (len - 1 - b)) == 0 { 0 } else { 1 }
                )?
            }
            writeln!(f, "| {} |", if *result { 1 } else { 0 })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() -> Result<()> {
        let result = TruthTable::try_from("A => Z")?;
        assert_eq!(result.variables, vec!['A', 'Z']);
        assert_eq!(result.results, vec![true, true, false, true]);
        Ok(())
    }

    #[test]
    fn error_invalid_rule() {
        let result = TruthTable::try_from("A = Z");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to evaluate permutations"
        );
    }
}
