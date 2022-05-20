use Token::*;

use anyhow::{anyhow, Context, Result};
use std::iter::Peekable;

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

fn get_direction<I>(lexer: &mut I, c: char) -> Result<Direction>
where
    I: Iterator<Item = char>,
{
    if let Some(next) = lexer.next() {
        match (c, next) {
            ('=', '>') => Ok(Direction::UniDirectional),
            ('<', '=') => match lexer.next() {
                Some('>') => Ok(Direction::BiDirectional),
                _ => Err(anyhow!("Unable to finish lexing implicator")),
            },
            _ => Err(anyhow!("Unable to finish lexing implicator")),
        }
    } else {
        Err(anyhow!("Unable to finish lexing implicator"))
    }
}

pub fn tokenize_rule(input: &str) -> Result<Vec<Token>> {
    let mut lexer = input.chars();
    let mut token_list: Vec<Token> = Vec::with_capacity(input.len());
    while let Some(c) = lexer.next() {
        match c {
            '(' | ')' => token_list.push(Parenthesis(c)),
            '!' | '+' | '|' | '^' => token_list.push(Operator(c)),
            '=' | '<' => token_list.push(Implicator(get_direction(&mut lexer, c)?)),
            '0' => token_list.push(Bool(false)),
            '1' => token_list.push(Bool(true)),
            c if c.is_whitespace() => {}
            _ => return Err(anyhow!("Unexpected character: {}", c)),
        }
    }
    Ok(token_list)
}

fn get_rule<'a, I>(token_list: &mut Peekable<I>) -> Result<bool>
where
    I: Iterator<Item = &'a Token>,
{
    let antecedent = get_operator(token_list)?;
    if let Some(implicator) = token_list.next() {
        let consequent = get_operator(token_list)?;
        match implicator {
            Implicator(direction) => match direction {
                Direction::UniDirectional => Ok(!antecedent | consequent),
                Direction::BiDirectional => Ok(antecedent == consequent),
            },
            _ => unreachable!(),
        }
    } else {
        Err(anyhow!("No implicator found"))
    }
}

fn get_operator<'a, I>(token_list: &mut Peekable<I>) -> Result<bool>
where
    I: Iterator<Item = &'a Token>,
{
    let mut node = get_factor(token_list);
    while let Some(Operator(_)) = token_list.peek() {
        node = match token_list.next() {
            Some(Operator('+')) => Ok(node? & get_factor(token_list)?),
            Some(Operator('|')) => Ok(node? | get_factor(token_list)?),
            Some(Operator('^')) => Ok(node? ^ get_factor(token_list)?),
            _ => unreachable!(),
        }
    }
    node
}

fn get_factor<'a, I>(token_list: &mut Peekable<I>) -> Result<bool>
where
    I: Iterator<Item = &'a Token>,
{
    match token_list.next() {
        Some(Parenthesis('(')) => {
            let res = get_operator(token_list);
            match token_list.next() {
                Some(Parenthesis(')')) => res,
                _ => Err(anyhow!("Missing closing parenthesis")),
            }
        }
        Some(Operator('!')) => Ok(!get_factor(token_list)?),
        Some(Bool(b)) => Ok(*b),
        Some(t) => Err(anyhow!("Invalid factor token '{:?}'", t)),
        None => Err(anyhow!("Unexpected end of token list")),
    }
}

pub fn evaluate_rule(input: &str) -> Result<bool> {
    let token_list =
        tokenize_rule(input).context(format!("Failed to tokenize input: '{}'", input))?;
    get_rule(&mut token_list.iter().peekable())
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn uni_directional() -> Result<()> {
        assert_eq!(evaluate_rule("1 => 0")?, false);
        assert_eq!(evaluate_rule("0 => 1")?, true);
        assert_eq!(evaluate_rule("1 => 1")?, true);
        assert_eq!(evaluate_rule("0 => 0")?, true);
        Ok(())
    }

    #[test]
    fn bi_directional() -> Result<()> {
        assert_eq!(evaluate_rule("1 <=> 0")?, false);
        assert_eq!(evaluate_rule("0 <=> 1")?, false);
        assert_eq!(evaluate_rule("1 <=> 1")?, true);
        assert_eq!(evaluate_rule("0 <=> 0")?, true);
        Ok(())
    }

    #[test]
    fn not() -> Result<()> {
        assert_eq!(evaluate_rule("!1 => 0")?, true);
        assert_eq!(evaluate_rule("!0 => 0")?, false);

        assert_eq!(evaluate_rule("1 => !1")?, false);
        assert_eq!(evaluate_rule("1 => !0")?, true);
        Ok(())
    }

    #[test]
    fn and() -> Result<()> {
        assert_eq!(evaluate_rule("1 + 1 => 0")?, false);
        assert_eq!(evaluate_rule("1 + 0 => 0")?, true);
        assert_eq!(evaluate_rule("0 + 1 => 0")?, true);
        assert_eq!(evaluate_rule("0 + 0 => 0")?, true);

        assert_eq!(evaluate_rule("1 => 1 + 1")?, true);
        assert_eq!(evaluate_rule("1 => 0 + 1")?, false);
        assert_eq!(evaluate_rule("1 => 1 + 0")?, false);
        assert_eq!(evaluate_rule("1 => 0 + 0")?, false);
        Ok(())
    }

    #[test]
    fn or() -> Result<()> {
        assert_eq!(evaluate_rule("1 | 1 => 0")?, false);
        assert_eq!(evaluate_rule("1 | 0 => 0")?, false);
        assert_eq!(evaluate_rule("0 | 1 => 0")?, false);
        assert_eq!(evaluate_rule("0 | 0 => 0")?, true);

        assert_eq!(evaluate_rule("1 => 1 | 1")?, true);
        assert_eq!(evaluate_rule("1 => 1 | 0")?, true);
        assert_eq!(evaluate_rule("1 => 0 | 1")?, true);
        assert_eq!(evaluate_rule("1 => 0 | 0")?, false);
        Ok(())
    }

    #[test]
    fn xor() -> Result<()> {
        assert_eq!(evaluate_rule("1 ^ 1 => 0")?, true);
        assert_eq!(evaluate_rule("1 ^ 0 => 0")?, false);
        assert_eq!(evaluate_rule("0 ^ 1 => 0")?, false);
        assert_eq!(evaluate_rule("0 ^ 0 => 0")?, true);

        assert_eq!(evaluate_rule("1 => 1 ^ 1")?, false);
        assert_eq!(evaluate_rule("1 => 1 ^ 0")?, true);
        assert_eq!(evaluate_rule("1 => 0 ^ 1")?, true);
        assert_eq!(evaluate_rule("1 => 0 ^ 0")?, false);
        Ok(())
    }

    #[test]
    fn parenthesis() -> Result<()> {
        assert_eq!(evaluate_rule("1 | 0 + 0 => 0")?, true);
        assert_eq!(evaluate_rule("(1 | 0) + 0 => 0")?, true);
        assert_eq!(evaluate_rule("1 | (0 + 0) => 0")?, false);
        assert_eq!(evaluate_rule("0 + 0 | 1 => 0")?, false);
        assert_eq!(evaluate_rule("(0 + 0) | 1 => 0")?, false);
        assert_eq!(evaluate_rule("0 + (0 | 1) => 0")?, true);

        assert_eq!(evaluate_rule("1 => 1 | 0 + 0")?, false);
        assert_eq!(evaluate_rule("1 => (1 | 0) + 0")?, false);
        assert_eq!(evaluate_rule("1 => 1 | (0 + 0)")?, true);
        assert_eq!(evaluate_rule("1 => 0 + 0 | 1")?, true);
        assert_eq!(evaluate_rule("1 => (0 + 0) | 1")?, true);
        assert_eq!(evaluate_rule("1 => 0 + (0 | 1)")?, false);
        Ok(())
    }

    #[test]
    fn error_empty() {
        let result = evaluate_rule("");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Unexpected end of token list"
        );
    }

    #[test]
    fn error_invalid_state() {
        let result = evaluate_rule("A => Z");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to tokenize input: 'A => Z'"
        );
    }

    #[test]
    fn error_invalid_operator() {
        let result = evaluate_rule("0 = 1");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to tokenize input: '0 = 1'"
        );
    }

    #[test]
    fn error_missing_operator_half() {
        let result = evaluate_rule("0 | => 0");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid factor token 'Implicator(UniDirectional)'"
        );
    }

    #[test]
    fn error_missing_implicator() {
        let result = evaluate_rule("0");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No implicator found");
    }

    #[test]
    fn error_missing_parenthesis() {
        let result = evaluate_rule("(0 => 0");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Missing closing parenthesis"
        );
    }
}
