use anyhow::{anyhow, Context, Result};
use std::iter::Peekable;
use Token::*;

macro_rules! node {
    ($token:expr) => {
        node!($token, Box::new(None), Box::new(None))
    };
    ($token:expr, $left:expr) => {
        node!($token, $left, Box::new(None))
    };
    ($token:expr, $left:expr, $right:expr) => {
        Box::new(Some(Node::new($token, $left, $right)))
    };
}

pub type Branch = Box<Option<Node>>;

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
    Attribute(char),
}

#[derive(Debug, Clone)]
pub struct Node {
    token: Token,
    left: Branch,
    right: Branch,
}

impl Node {
    pub fn new(token: Token, left: Branch, right: Branch) -> Node {
        Node { token, left, right }
    }
}

pub struct Parser;

impl<'a> Parser {
    pub fn new() -> Self {
        Parser {}
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
                    _ => Err(anyhow!("Unable to finish reading bidirectional inplicator")),
                },
                _ => Err(anyhow!("Unable to finish reading inplicator")),
            }
        } else {
            Err(anyhow!("Unable to finish reading inplicator"))
        }
    }

    pub fn tokenize(&mut self, input: &str) -> Result<Vec<Token>> {
        let mut lexer = input.chars().peekable();
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(c) = lexer.next() {
            match c {
                '(' | ')' => tokens.push(Parenthesis(c)),
                '!' | '+' | '|' | '^' => tokens.push(Operator(c)),
                '=' | '<' => tokens.push(Implicator(self.get_direction(&mut lexer, c)?)),
                'A'..='Z' => tokens.push(Attribute(c)),
                c if c.is_whitespace() => {}
                _ => return Err(anyhow!("Unexpected character: {}", c)),
            }
        }
        Ok(tokens)
    }

    fn get_rule<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Branch>
    where
        I: Iterator<Item = &'a Token>,
    {
        let lhs = self.get_operator(tokens);
        match tokens.peek() {
            Some(Implicator(_)) => {
                let token = tokens.next().context("Unexpected end of token list")?;
                let rhs = self.get_operator(tokens);
                match tokens.next() {
                    None => Ok(node!(*token, lhs?, rhs?)),
                    Some(t) => Err(anyhow!("Found unexpected token: {:?}", t)),
                }
            }
            _ => Err(anyhow!("No implicator found")),
        }
    }

    fn get_operator<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Branch>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut token = self.get_factor(tokens);
        loop {
            match tokens.peek() {
                Some(Operator('+')) | Some(Operator('|')) | Some(Operator('^')) => {
                    let parent = tokens.next().context("Unexpected end of token list")?;
                    let rhs = self.get_operator(tokens);
                    token = Ok(node!(*parent, token?, rhs?));
                }
                _ => break,
            }
        }
        token
    }

    fn get_factor<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Branch>
    where
        I: Iterator<Item = &'a Token>,
    {
        let token = tokens.next();
        match token {
            Some(Parenthesis('(')) => {
                let child = self.get_operator(tokens);
                match tokens.next() {
                    Some(Parenthesis(')')) => child,
                    _ => Err(anyhow!("Missing closing parenthesis")),
                }
            }
            Some(Operator('!')) => Ok(node!(
                *token.context("Unexpected end of tokenlist")?,
                self.get_factor(tokens)?
            )),
            Some(Attribute(_)) => Ok(node!(*token.context("Unexpected end of token list")?)),
            _ => Err(anyhow!("Unexpected end of token list")),
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<Branch> {
        let tokens = self
            .tokenize(input)
            .context(format!("Could not tokenize input"))?;

        Ok(self
            .get_rule(&mut tokens.iter().peekable())
            .context(format!("Could not parse"))?)
    }
}

#[cfg(test)]
mod tests {
    use crate::Parser;

    #[test]
    fn valid_input() {
        let mut parser = Parser::new();

        assert!(parser
            .tokenize("A+B <=> !C+   ((D ^ E) + (F | (G ^ !H)))")
            .is_ok());
        assert_eq!(
            parser
                .tokenize("A+B <=> !C+   ((D ^ E) + (F | (G ^ !H)))")
                .expect("")
                .len(),
            25
        );
        assert!(parser.tokenize("A+B          => C").is_ok());
        assert_eq!(parser.tokenize("A+B          => C").expect("").len(), 5);
        assert!(parser.tokenize("((A+B))          => C").is_ok());
        assert_eq!(parser.tokenize("((A+B))          => C").expect("").len(), 9);
        assert!(parser.tokenize("!A+!B          => (C^(!D))").is_ok());
        assert_eq!(
            parser
                .tokenize("!A+!B          => (C^(!D))")
                .expect("")
                .len(),
            14
        );
        assert!(parser.tokenize("!A<=>B").is_ok());
        assert_eq!(parser.tokenize("!A<=>B").expect("").len(), 4);
        assert!(parser.tokenize("!A+!B ^ C | D + E         => F").is_ok());
        assert_eq!(
            parser
                .tokenize("!A+!B ^ C | D + E         => F")
                .expect("")
                .len(),
            13
        );
        assert!(parser
            .tokenize("A+B <=> !C+   ((D ^ E) + (F | (G ^ !H)))         ^ I | (J + (!K))")
            .is_ok());
        assert_eq!(
            parser
                .tokenize("A+B <=> !C+   ((D ^ E) + (F | (G ^ !H)))         ^ I | (J + (!K))")
                .expect("")
                .len(),
            36
        );
    }

    #[test]
    fn invalid_input() {
        let mut parser = Parser::new();

        assert!(parser.tokenize("a").is_err());
        assert!(parser.tokenize("1").is_err());
        assert!(parser.tokenize("&").is_err());
        assert!(parser.tokenize("A => B\0 + C").is_err());
    }

    #[test]
    fn valid_tokenlist() {
        let mut parser = Parser::new();

        assert!(parser
            .parse("A+B <=> !C+   ((D ^ E) + (F | (G ^ !H)))")
            .is_ok());
        assert!(parser.parse("A+B          => C").is_ok());
        assert!(parser.parse("((A+B))          => C").is_ok());
        assert!(parser.parse("!A+!B          => (C^(!D))").is_ok());
        assert!(parser.parse("!A<=>B").is_ok());
        assert!(parser.parse("!A+!B ^ C | D + E         => F").is_ok());
        assert!(parser
            .parse("A+B <=> !C+   ((D ^ E) + (F | (G ^ !H)))         ^ I | (J + (!K))")
            .is_ok());
        assert!(parser.parse("A => !!!B").is_ok());
    }

    #[test]
    fn invalid_tokenlist() {
        let mut parser = Parser::new();

        assert!(parser.parse("=>").is_err());
        assert!(parser.parse("A").is_err());
        assert!(parser.parse("(").is_err());
        assert!(parser.parse(")").is_err());
        assert!(parser.parse("+").is_err());
        assert!(parser.parse("!").is_err());
        assert!(parser.parse("A => B => C").is_err());
        assert!(parser.parse("(A + (B!)C").is_err());
        assert!(parser.parse("A + (B!)C").is_err());
        assert!(parser.parse("A = B").is_err());
        assert!(parser
            .parse("A+B <=> !C+   ((D ^ E) + (F | (G ^ !H))         ^ I | (J + (!K))")
            .is_err());
        assert!(parser.parse("A !=> B").is_err());
        assert!(parser.parse("A => B!").is_err());
        assert!(parser.parse("A++B => C").is_err());
        assert!(parser.parse("A+B").is_err());
    }
}
