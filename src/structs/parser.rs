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

#[derive(Debug)]
pub struct Node {
    _token: Token,
    _left: Branch,
    _right: Branch,
}

impl Node {
    pub fn new(_token: Token, _left: Branch, _right: Branch) -> Node {
        Node {
            _token,
            _left,
            _right,
        }
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
                'A'..='Z' => tokenlist.push(Attribute(c)),
                c if c.is_whitespace() => {}
                _ => return Err(anyhow!("Unexpected character: {}", c)),
            }
        }
        Ok(tokenlist)
    }

    fn get_rule<I>(&mut self, tokenlist: &mut Peekable<I>) -> Result<Branch>
    where
        I: Iterator<Item = &'a Token>,
    {
        let antecedent = self.get_operator(tokenlist);
        match tokenlist.peek() {
            Some(Implicator(_)) => {
                let token = tokenlist.next().context("Unexpected end of token list")?;
                let consequent = self.get_operator(tokenlist);
                match tokenlist.next() {
                    None => Ok(node!(*token, antecedent?, consequent?)),
                    Some(t) => Err(anyhow!("Found unexpected token: {:?}", t)),
                }
            }
            _ => Err(anyhow!("No implicator found")),
        }
    }

    fn get_operator<I>(&mut self, tokenlist: &mut Peekable<I>) -> Result<Branch>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut node = self.get_factor(tokenlist);
        while let Some(Operator('+')) | Some(Operator('|')) | Some(Operator('^')) = tokenlist.peek()
        {
            node = Ok(node!(
                *tokenlist.next().context("Unexpected end of token list")?,
                node?,
                self.get_operator(tokenlist)?
            ));
        }
        node
    }

    fn get_factor<I>(&mut self, tokenlist: &mut Peekable<I>) -> Result<Branch>
    where
        I: Iterator<Item = &'a Token>,
    {
        let token = tokenlist.next();
        match token {
            Some(Parenthesis('(')) => {
                let node = self.get_operator(tokenlist);
                match tokenlist.next() {
                    Some(Parenthesis(')')) => node,
                    _ => Err(anyhow!("Missing closing parenthesis")),
                }
            }
            Some(Operator('!')) => Ok(node!(
                *token.context("Unexpected end of token list")?,
                self.get_factor(tokenlist)?
            )),
            Some(Attribute(_)) => Ok(node!(*token.context("Unexpected end of token list")?)),
            _ => Err(anyhow!("Unexpected end of token list")),
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<Branch> {
        let tokenlist = self.tokenize(input).context("Could not tokenize input")?;
        let tree = self
            .get_rule(&mut tokenlist.iter().peekable())
            .context("Syntactical error")?;
        Ok(tree)
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

        assert!(parser.parse("").is_err());
        assert!(parser.parse(" ").is_err());
        assert!(parser.parse("=>").is_err());
        assert!(parser.parse("=> B").is_err());
        assert!(parser.parse("A =>").is_err());
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
        assert!(parser.parse("A+|B => C").is_err());
        assert!(parser.parse("A+B").is_err());
    }
}
