use super::*;

use anyhow::{Context, Result};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

// Structure that holds key pairs of identifier and all related truth tables.
#[derive(Default)]
pub struct RuleMap {
    map: HashMap<char, HashSet<Rc<TruthTable>>>,
}

impl RuleMap {
    // Inserts a new rule in the rulemap Ad-Hoc
    pub fn insert<T>(&mut self, rule: T) -> Result<()>
    where
        T: Borrow<str>,
    {
        let ptr = Rc::new(
            TruthTable::try_from(PermutationIter::new(rule.borrow())).context(format!(
                "Failed to create truth table from: '{}'",
                rule.borrow()
            ))?,
        );
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
        T: Borrow<str>,
    {
        for rule in rules.iter() {
            self.insert(rule.borrow())?
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
        let mut map = RuleMap::default();
        map.insert_vec(rules)?;
        Ok(map)
    }
}

impl fmt::Debug for RuleMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut map = self.map.iter().peekable();
        while let Some((k, v)) = map.next() {
            writeln!(f, "{}", k)?;
            let mut table = v.iter().peekable();
            while let Some(t) = table.next() {
                if map.peek().is_none() && table.peek().is_none() {
                    write!(f, "{:?}", t)?;
                } else {
                    writeln!(f, "{:?}", t)?;
                }
            }
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
    fn empty() -> Result<()> {
        let result = RuleMap::try_from(Vec::<String>::new())?;
        assert!(result.map.is_empty());
        Ok(())
    }

    #[test]
    fn from() -> Result<()> {
        let result = RuleMap::try_from(vec!["A => B", "B => C"])?;
        assert_eq!(result.map.len(), 3);
        assert_eq!(result.map.get(&'A').map_or(0, |v| v.len()), 1);
        assert_eq!(result.map.get(&'B').map_or(0, |v| v.len()), 2);
        assert_eq!(result.map.get(&'C').map_or(0, |v| v.len()), 1);
        Ok(())
    }

    #[test]
    fn insert() -> Result<()> {
        let mut result = RuleMap::try_from(Vec::<String>::new())?;
        result.insert("A => B")?;
        assert_eq!(result.map.len(), 2);
        assert_eq!(result.map.get(&'A').map_or(0, |v| v.len()), 1);
        assert_eq!(result.map.get(&'B').map_or(0, |v| v.len()), 1);

        result.insert("B => C")?;
        assert_eq!(result.map.len(), 3);
        assert_eq!(result.map.get(&'A').map_or(0, |v| v.len()), 1);
        assert_eq!(result.map.get(&'B').map_or(0, |v| v.len()), 2);
        assert_eq!(result.map.get(&'C').map_or(0, |v| v.len()), 1);
        Ok(())
    }

    #[test]
    fn insert_vec() -> Result<()> {
        let mut result = RuleMap::try_from(Vec::<String>::new())?;
        result.insert_vec(vec!["A => B", "B => C"])?;
        assert_eq!(result.map.len(), 3);
        assert_eq!(result.map.get(&'A').map_or(0, |v| v.len()), 1);
        assert_eq!(result.map.get(&'B').map_or(0, |v| v.len()), 2);
        assert_eq!(result.map.get(&'C').map_or(0, |v| v.len()), 1);
        Ok(())
    }

    #[test]
    fn error_invalid_rule() {
        let result = RuleMap::try_from(vec!["A =>"]);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to create truth table from: 'A =>'"
        );
    }
}
