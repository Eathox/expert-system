use crate::*;

use anyhow::{anyhow, Context, Result};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::iter::Peekable;
use std::rc::Rc;
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

#[derive(Default)]
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
pub struct PermutationIter {
    formula: String,
    pub variables: Vec<char>,
    size: usize,
}

impl PermutationIter {
    pub fn new(formula: impl AsRef<str>) -> PermutationIter {
        let formula = formula.as_ref().to_owned();
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

// TruthTable struct holds the truth table data of an input rule.
// It can be constructed using a PermutationIter. Since the permutations generated by
// PermuationIter is always guaranteed to follow the same pattern, the order of the results
// implicitly holds the propositional data for each entry in the TruthTable. Example:
// `0 => 0` implies index 0b00, results[0]
// `0 => 1` implies index 0b01, results[1]
// `1 => 0` implies index 0b10, results[2]
// `1 => 1` implies index 0b11, results[3]
#[derive(Default, Eq, PartialEq, Hash, Clone)]
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

impl TryFrom<PermutationIter> for TruthTable {
    type Error = anyhow::Error;

    fn try_from(mut permutation_iter: PermutationIter) -> Result<Self, Self::Error> {
        let mut table = Self::new();
        let mut parser = RuleParser::new();
        for permutation in permutation_iter.by_ref() {
            table.results.push(
                parser
                    .evaluate(&permutation)
                    .context(format!("Failed to evaluate permutation {}", permutation))?,
            );
        }
        table.variables.append(&mut permutation_iter.variables);
        Ok(table)
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

// Structure that maps for all identifiers the related truth tables.
#[derive(Default, Eq, PartialEq)]
struct RuleMap {
    map: HashMap<char, HashSet<Rc<TruthTable>>>,
}

impl RuleMap {
    // To insert a new rule in the rulemap Ad-Hoc
    pub fn insert<T>(&mut self, rule: T) -> Result<()>
    where
        T: AsRef<str>,
    {
        let ptr = Rc::new(TruthTable::try_from(PermutationIter::new(rule))?);
        for v in ptr.variables.iter() {
            let tables = self
                .map
                .entry(*v)
                .or_insert_with(|| HashSet::from([Rc::clone(&ptr)]));
            tables.insert(Rc::clone(&ptr));
        }
        Ok(())
    }

    pub fn insert_vec<T>(&mut self, rules: Vec<T>) -> Result<()>
    where
        T: AsRef<str>,
    {
        for rule in rules.iter() {
            self.insert(rule)?
        }
        Ok(())
    }
}

impl<T> TryFrom<Vec<T>> for RuleMap
where
    T: Borrow<str>,
{
    type Error = anyhow::Error;

    fn try_from(rules: Vec<T>) -> Result<Self> {
        let rule_set = rules
            .into_iter()
            .map(|s| TruthTable::try_from(PermutationIter::new(s.borrow())))
            .collect::<Result<Vec<TruthTable>>>()
            .context("Unable to build RuleMap")?;
        let mut map = HashMap::new();
        for rule in rule_set.into_iter() {
            let ptr = Rc::new(rule);
            for v in ptr.variables.iter() {
                let tables = map
                    .entry(*v)
                    .or_insert_with(|| HashSet::from([Rc::clone(&ptr)]));
                tables.insert(Rc::clone(&ptr));
            }
        }
        Ok(RuleMap { map })
    }
}

impl fmt::Display for RuleMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (k, v) in self.map.iter() {
            writeln!(f, "{}", k)?;
            for t in v.iter() {
                writeln!(f, "{}", t)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod purmutation_iter {
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
    }

    #[test]
    fn respect_white_space() {
        let mut iter = PermutationIter::new("\t\n\r A");
        assert_eq!(Some("\t\n\r 0".to_string()), iter.next());
        assert_eq!(Some("\t\n\r 1".to_string()), iter.next());
        assert_eq!(None, iter.next());
    }
}

#[cfg(test)]
mod truth_table {
    use super::*;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() -> Result<()> {
        let table = TruthTable::try_from(PermutationIter::new("A => Z"))?;
        assert_eq!(table.variables, vec!['A', 'Z']);
        assert_eq!(table.results, vec![true, true, false, true]);
        Ok(())
    }

    #[test]
    fn test_valid_rule() -> Result<()> {
        let table = TruthTable::try_from(PermutationIter::new("A + B <=> C"))?;
        assert_eq!(table.variables, vec!['A', 'B', 'C']);
        assert_eq!(
            table.results,
            vec![true, false, true, false, true, false, false, true]
        );
        Ok(())
    }
}

#[cfg(test)]
mod rulemap {
    use super::*;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty() -> Result<()> {
        Ok(())
    }

    #[test]
    fn insert() -> Result<()> {
        Ok(())
    }

    #[test]
    fn insert_vec() -> Result<()> {
        Ok(())
    }
}
