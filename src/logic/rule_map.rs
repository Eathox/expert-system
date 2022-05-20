use super::TruthTable;

use anyhow::{Context, Result};
use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    fmt,
    rc::Rc,
};

// Structure that holds key pairs of identifier and all related truth tables.
#[derive(Default)]
pub struct RuleMap {
    map: HashMap<char, HashSet<Rc<TruthTable>>>,
}

impl RuleMap {
    pub fn insert(&mut self, table: TruthTable) {
        let ptr = Rc::new(table);
        for v in ptr.variables.iter() {
            let tables = self.map.entry(*v).or_insert_with(|| HashSet::new());
            tables.insert(Rc::clone(&ptr));
        }
    }

    pub fn insert_vec(&mut self, tables: Vec<TruthTable>) {
        for table in tables {
            self.insert(table);
        }
    }

    pub fn insert_rule<T>(&mut self, rule: T) -> Result<()>
    where
        T: Borrow<str>,
    {
        let table = TruthTable::try_from(rule.borrow()).context(format!(
            "Failed to create truth table from: '{}'",
            rule.borrow()
        ))?;
        self.insert(table);
        Ok(())
    }

    pub fn insert_rule_vec<T>(&mut self, rules: Vec<T>) -> Result<()>
    where
        T: Borrow<str>,
    {
        for rule in rules {
            self.insert_rule(rule.borrow())?;
        }
        Ok(())
    }
}

impl From<Vec<TruthTable>> for RuleMap {
    fn from(tables: Vec<TruthTable>) -> Self {
        let mut map = RuleMap::default();
        map.insert_vec(tables);
        map
    }
}

impl TryFrom<Vec<&str>> for RuleMap {
    type Error = anyhow::Error;

    fn try_from(rules: Vec<&str>) -> Result<Self> {
        let mut map = RuleMap::default();
        map.insert_rule_vec(rules)?;
        Ok(map)
    }
}

impl TryFrom<Vec<String>> for RuleMap {
    type Error = anyhow::Error;

    fn try_from(rules: Vec<String>) -> Result<Self> {
        let mut map = RuleMap::default();
        map.insert_rule_vec(rules)?;
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
    fn empty() {
        let result = RuleMap::default();
        assert!(result.map.is_empty());
    }

    #[test]
    fn from() {
        let result = RuleMap::from(vec![
            TruthTable {
                variables: vec!['A', 'B'],
                results: vec![],
            },
            TruthTable {
                variables: vec!['B', 'C'],
                results: vec![],
            },
        ]);
        assert_eq!(result.map.len(), 3);
        assert_eq!(result.map.get(&'A').map_or(0, |v| v.len()), 1);
        assert_eq!(result.map.get(&'B').map_or(0, |v| v.len()), 2);
        assert_eq!(result.map.get(&'C').map_or(0, |v| v.len()), 1);
    }

    #[test]
    fn insert() {
        let mut result = RuleMap::default();
        result.insert(TruthTable {
            variables: vec!['A', 'B'],
            results: vec![],
        });
        assert_eq!(result.map.len(), 2);
        assert_eq!(result.map.get(&'A').map_or(0, |v| v.len()), 1);
        assert_eq!(result.map.get(&'B').map_or(0, |v| v.len()), 1);

        result.insert(TruthTable {
            variables: vec!['B', 'C'],
            results: vec![],
        });
        assert_eq!(result.map.len(), 3);
        assert_eq!(result.map.get(&'A').map_or(0, |v| v.len()), 1);
        assert_eq!(result.map.get(&'B').map_or(0, |v| v.len()), 2);
        assert_eq!(result.map.get(&'C').map_or(0, |v| v.len()), 1);
    }

    #[test]
    fn insert_vec() {
        let mut result = RuleMap::default();
        result.insert_vec(vec![
            TruthTable {
                variables: vec!['A', 'B'],
                results: vec![],
            },
            TruthTable {
                variables: vec!['B', 'C'],
                results: vec![],
            },
        ]);
        assert_eq!(result.map.len(), 3);
        assert_eq!(result.map.get(&'A').map_or(0, |v| v.len()), 1);
        assert_eq!(result.map.get(&'B').map_or(0, |v| v.len()), 2);
        assert_eq!(result.map.get(&'C').map_or(0, |v| v.len()), 1);
    }

    #[test]
    fn from_rule() -> Result<()> {
        let result = RuleMap::try_from(vec!["A => B", "B => C"])?;
        assert_eq!(result.map.len(), 3);
        assert_eq!(result.map.get(&'A').map_or(0, |v| v.len()), 1);
        assert_eq!(result.map.get(&'B').map_or(0, |v| v.len()), 2);
        assert_eq!(result.map.get(&'C').map_or(0, |v| v.len()), 1);
        Ok(())
    }

    #[test]
    fn insert_rule() -> Result<()> {
        let mut result = RuleMap::default();
        result.insert_rule("A => B")?;
        assert_eq!(result.map.len(), 2);
        assert_eq!(result.map.get(&'A').map_or(0, |v| v.len()), 1);
        assert_eq!(result.map.get(&'B').map_or(0, |v| v.len()), 1);

        result.insert_rule("B => C")?;
        assert_eq!(result.map.len(), 3);
        assert_eq!(result.map.get(&'A').map_or(0, |v| v.len()), 1);
        assert_eq!(result.map.get(&'B').map_or(0, |v| v.len()), 2);
        assert_eq!(result.map.get(&'C').map_or(0, |v| v.len()), 1);
        Ok(())
    }

    #[test]
    fn insert_rule_vec() -> Result<()> {
        let mut result = RuleMap::default();
        result.insert_rule_vec(vec!["A => B", "B => C"])?;
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
