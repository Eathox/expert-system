use anyhow::{anyhow, Context, Result};
use std::iter::Peekable;

pub type Child = Box<Option<Node>>;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Unidirectional,
    Bidirectional,
}

#[derive(Debug, Copy, Clone)]
pub enum Token {
    Implicator(Direction),
    Operator(char),
    Parenthesis(char),
    Identifier(char),
}

#[derive(Debug, Clone)]
pub struct Node {
    token: Token,
    left: Child,
    right: Child,
}

impl Node {
    pub fn new(token: Token, left: Child, right: Child) -> Node {
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
                ('=', '>') => Ok(Direction::Unidirectional),
                ('<', '=') => match lexer.next() {
                    Some('>') => Ok(Direction::Bidirectional),
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
                '(' | ')' => tokens.push(Token::Parenthesis(c)),
                '!' | '+' | '|' | '^' => tokens.push(Token::Operator(c)),
                '=' | '<' => tokens.push(Token::Implicator(self.get_direction(&mut lexer, c)?)),
                'A'..='Z' => tokens.push(Token::Identifier(c)),
                c if c.is_whitespace() => {}
                _ => return Err(anyhow!("Unexpected character: {}", c)),
            }
        }
        Ok(tokens)
    }

    fn get_rule<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Child>
    where
        I: Iterator<Item = &'a Token>,
    {
        let lhs = self.get_operator(tokens);
        match tokens.peek() {
            Some(Token::Implicator(_)) => {
                let token = tokens.next().unwrap();
                let rhs = self.get_rule(tokens);
                Ok(Box::new(Some(Node::new(*token, lhs?, rhs?))))
            }
            _ => lhs,
        }
    }

    fn get_operator<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Child>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut token = self.get_factor(tokens);
        loop {
            match tokens.peek() {
                Some(Token::Operator('+'))
                | Some(Token::Operator('|'))
                | Some(Token::Operator('^')) => {
                    let parent = tokens.next().unwrap();
                    let rhs = self.get_operator(tokens);
                    token = Ok(Box::new(Some(Node::new(*parent, token?, rhs?))));
                }
                _ => break,
            }
        }
        token
    }

    fn get_factor<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Child>
    where
        I: Iterator<Item = &'a Token>,
    {
        let token = tokens.next();
        match token {
            Some(Token::Parenthesis('(')) => {
                let child = self.get_operator(tokens);
                match tokens.next() {
                    Some(Token::Parenthesis(')')) => child,
                    _ => Err(anyhow!("Missing closing parenthesis")),
                }
            }
            Some(Token::Operator('!')) => {
                let child = self.get_factor(tokens);
                Ok(Box::new(Some(Node::new(
                    *token.unwrap(),
                    child?,
                    Box::new(None),
                ))))
            }
            Some(Token::Identifier(_)) => Ok(Box::new(Some(Node::new(
                *token.unwrap(),
                Box::new(None),
                Box::new(None),
            )))),
            _ => Err(anyhow!("Unexpected end of token list")),
        }
    }

    pub fn parse(&mut self) -> Result<Child> {
        let tokens = self
            .tokenize("A+B <=")
            .context(format!("Could not tokenize input"))?;

        let tree = self
            .get_rule(&mut tokens.iter().peekable())
            .context(format!("Could not parse"))?;

        Ok(tree)
    }
}
