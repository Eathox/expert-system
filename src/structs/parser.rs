use anyhow::{Context, Result, anyhow};
use std::fs::File;
use std::iter::Peekable;

pub type Child<'a> = Box<Option<Node<'a>>>;

#[derive(Debug, Clone)]
pub enum Token {
    Implicator,
    Operator(char),
    Parenthesis(char),
    Identifier(char),
}

#[derive(Debug, Clone)]
pub struct Node<'a> {
    token: &'a Token,
    left: Child<'a>,
    right: Child<'a>,
}

impl Node<'_> {
    pub fn new<'a>(token: &'a Token, left: Child<'a>, right: Child<'a>) -> Node<'a> {
        Node { token, left, right }
    }
}

pub struct Parser {
}

impl<'a> Parser {
    pub fn new() -> Self {
        Parser {} //{ tokens: Vec::new() }
    }

	pub fn tokenize(&mut self, rule: &str) -> Result<Vec<Token>> {
		let mut lexer = rule.chars().peekable();
		let mut tokens: Vec<Token> = Vec::new();

		while let Some(c) = lexer.next() {
			match c {
				'(' | ')' => tokens.push(Token::Parenthesis(c)),
				'!' | '+' | '|' | '^' => tokens.push(Token::Operator(c)),
				'>' => tokens.push(Token::Implicator),
				'A'..='Z' => tokens.push(Token::Identifier(c)),
				//TODO: Modify Implicator
				c if c.is_whitespace() => {}
				_ => return Err(anyhow!("Unexpected character: {}", c)),
			}
		}
		Ok(tokens)
	}

    fn get_implicator<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Child<'a>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let lhs = self.get_operator(tokens);
        match tokens.peek() {
			Some(Token::Implicator) => {
				let token = tokens.next().unwrap();
				let rhs = self.get_implicator(tokens);
				Ok(Box::new(Some(Node::new(token, lhs?, rhs?))))
			}
			_ => lhs,
		}
    }

    fn get_operator<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Child<'a>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut token = self.get_factor(tokens);
        loop {
            match tokens.peek() {
                Some(Token::Operator('+')) | Some(Token::Operator('|')) | Some(Token::Operator('^')) => {
                    let parent = tokens.next().unwrap();
                    let rhs = self.get_operator(tokens);
                    token = Ok(Box::new(Some(Node::new(parent, token?, rhs?))));
                },
                _ => break
            }
        };
        token
    }

    fn get_factor<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Child<'a>>
    where
        I: Iterator<Item = &'a Token>,
    {
        let token = tokens.next();
        match token {
            Some(Token::Parenthesis('(')) => {
                let child = self.get_implicator(tokens);
                match tokens.next() {
                    Some(Token::Parenthesis(')')) => child,
                    _ => panic!()
                }
            },
            Some(Token::Identifier(c)) => Ok(Box::new(Some(Node::new(token.unwrap(), Box::new(None), Box::new(None))))),
            _ => panic!()
        }
    }

    // fn get_identifier<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Child<'a>>
    // where
    //     I: Iterator<Item = &'a Token>,
    // {
    //     // Dummy Error
    //     File::open(&"dummy").context(format!("Could find identifier: {}", "Line 4"))?;

    //     // implementation goes here
    //     todo!();
    // }

    pub fn parse(&mut self) -> Result<()> {
        // let tokens = self.tokenize("A+B    > C | D").unwrap();
        let tokens = self.tokenize("A+B>     C|     (D ^ E) ").unwrap();

        let tree = self.get_implicator(&mut tokens.iter().peekable())
            .context(format!("Could not parse: {}", "Line 4"))?;

        println!("{:#?}", tree);

        Ok(())
    }
}
