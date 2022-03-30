use anyhow::{anyhow, Context, Result};
use std::collections::HashSet;
use std::fmt;
use std::iter::Peekable;
use Token::*;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    UniDirectional,
    BiDirectional,
}

#[derive(Debug, Copy, Clone)]
pub enum Token {
    Implicator(Direction),
    Operator(char),
    Parenthesis(char),
    Bool(bool),
}

pub struct RuleParser;

impl<'a> RuleParser {
    pub fn new() -> Self {
        RuleParser {}
    }

    fn get_direction<I>(&mut self, lexer: &mut Peekable<I>, c: char) -> Result<Direction>
    where
        I: Iterator<Item = char>,
    {
        if let Some(next) = lexer.next() {
            match (c, next) {
                ('=', '>') => Ok(Direction::UniDirectional),
                ('<', '=') => match lexer.next() {
                    Some('>') => Ok(Direction::BiDirectional),
                    _ => Err(anyhow!("Unable to finish lexing inplicator")),
                },
                _ => Err(anyhow!("Unable to finish lexing inplicator")),
            }
        } else {
            Err(anyhow!("Unable to finish lexing inplicator"))
        }
    }

    pub fn tokenize(&mut self, input: &str) -> Result<Vec<Token>> {
        let mut lexer = input.chars().peekable();
        let mut tokenlist: Vec<Token> = Vec::new();
        while let Some(c) = lexer.next() {
            match c {
                '(' | ')' => tokenlist.push(Parenthesis(c)),
                '!' | '+' | '|' | '^' => tokenlist.push(Operator(c)),
                '=' | '<' => tokenlist.push(Implicator(self.get_direction(&mut lexer, c)?)),
                '0' => tokenlist.push(Bool(false)),
                '1' => tokenlist.push(Bool(true)),
                c if c.is_whitespace() => {}
                _ => return Err(anyhow!("Unexpected character: {}", c)),
            }
        }
        Ok(tokenlist)
    }

    fn get_rule<I>(&mut self, tokenlist: &mut Peekable<I>) -> Result<bool>
    where
        I: Iterator<Item = &'a Token>,
    {
        let antecedent = self.get_operator(tokenlist)?;
        if let Some(implicator) = tokenlist.next() {
            let consequent = self.get_operator(tokenlist)?;
            match implicator {
                Implicator(direction) => match direction {
                    Direction::UniDirectional => Ok(!antecedent | consequent),
                    Direction::BiDirectional => Ok(antecedent == consequent),
                },
                _ => Err(anyhow!("No implicator found")),
            }
        } else {
            Err(anyhow!("Unexpected end of token list"))
        }
    }

    fn get_operator<I>(&mut self, tokenlist: &mut Peekable<I>) -> Result<bool>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut node = self.get_factor(tokenlist);
        while let Some(Operator(op)) = tokenlist.peek() {
            node = match tokenlist.next() {
                Some(Operator('+')) => Ok(node? & self.get_factor(tokenlist)?),
                Some(Operator('|')) => Ok(node? | self.get_factor(tokenlist)?),
                Some(Operator('^')) => Ok(node? ^ self.get_factor(tokenlist)?),
                _ => Err(anyhow!("Found unexpected operator: {:?}", op)),
            }
        }
        node
    }

    fn get_factor<I>(&mut self, tokenlist: &mut Peekable<I>) -> Result<bool>
    where
        I: Iterator<Item = &'a Token>,
    {
        match tokenlist.next() {
            Some(Parenthesis('(')) => {
                let res = self.get_operator(tokenlist);
                match tokenlist.next() {
                    Some(Parenthesis(')')) => res,
                    _ => Err(anyhow!("Missing closing parenthesis")),
                }
            }
            Some(Operator('!')) => Ok(!self.get_factor(tokenlist)?),
            Some(Bool(b)) => Ok(*b),
            _ => Err(anyhow!("Unexpected end of token list")),
        }
    }

    pub fn evaluate(&mut self, input: &str) -> Result<bool> {
        let tokenlist = self.tokenize(input).context("Could not tokenize input")?;
        self.get_rule(&mut tokenlist.iter().peekable())
            .context("Syntactical error")
    }
}

// PermutationIter is an iterator that iterates over all permutations of a rule input string
// The order in which the permutations are generated is always following the same pattern, example:
// `A => B` produces the following permutations:
// `0 => 0`
// `0 => 1`
// `1 => 0`
// `1 => 1`
pub struct PermutationIter<'a> {
    formula: &'a str,
    pub variables: Vec<char>,
    size: usize,
}

impl PermutationIter<'_> {
    pub fn new(formula: &str) -> PermutationIter {
        let mut set = HashSet::new();
        let mut variables = formula
            .chars()
            .filter_map(|c| match c {
                'A'..='Z' if set.insert(c) => Some(c),
                _ => None,
            })
            .collect::<Vec<char>>();
        variables.sort_unstable();
        PermutationIter {
            formula,
            variables,
            size: 0,
        }
    }
}

impl Iterator for PermutationIter<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 1 << self.variables.len() {
            None
        } else {
            let mut permutation = String::from(self.formula);
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

// TruthTable struct holds the truth table data of an input rule.
// It can be constructed using a PermutationIter. Since the permutations generated by
// PermuationIter is always guaranteed to follow the same pattern, the order of the results
// implicitly holds the propositional data for each entry in the TruthTable. Example:
// `0 => 0` implies index 0b00, results[0]
// `0 => 1` implies index 0b01, results[1]
// `1 => 0` implies index 0b10, results[2]
// `1 => 1` implies index 0b11, results[3]
#[derive(PartialEq)]
pub struct TruthTable {
    pub variables: Vec<char>,
    pub results: Vec<bool>,
}

impl TruthTable {
    pub fn new() -> Self {
        TruthTable {
            variables: Vec::new(),
            results: Vec::new(),
        }
    }
}

impl From<PermutationIter<'_>> for TruthTable {
    fn from(mut permutationlist: PermutationIter) -> Self {
        let mut table = Self::new();
        let mut parser = RuleParser::new();
        for permutation in permutationlist.by_ref() {
            table.results.push(parser.evaluate(&permutation).unwrap());
        }
        table.variables.append(&mut permutationlist.variables);
        table
    }
}

impl fmt::Display for TruthTable {
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
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_permutationiter() {
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
    fn test_truthtable() {
        let table = TruthTable::from(PermutationIter::new("A + B <=> C"));
        assert_eq!(table.variables, vec!['A', 'B', 'C']);
        assert_eq!(
            table.results,
            vec![true, false, true, false, true, false, false, true]
        );

        let table = TruthTable::from(PermutationIter::new("A + D <=> C | X"));
        assert_eq!(table.variables, vec!['A', 'C', 'D', 'X']);
        assert_eq!(
            table.results,
            vec![
                true, false, true, false, false, false, false, false, true, false, false, true,
                false, false, true, true
            ]
        );
    }
}
