use anyhow::{Context, Result, anyhow};
use expert_system::*;
use std::fs::File;
use std::iter::Peekable;


pub type Child<'a> = Box<Option<Node<'a>>>;

pub struct Implicator;
pub struct Operator(char);
pub struct Parenthesis(char);
pub struct Identifier(char);

pub enum Token {
	Implicator(Implicator),
	Operator(Operator),
	Parenthesis(Parenthesis),
	Identifier(Identifier),
}

pub trait FromToken {
	fn get<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<Child>
	where
		I: Iterator<Item = &'a Token>, ;
}

impl FromToken for Implicator {
	fn get<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<Child>
	where
		I: Iterator<Item = &'a Token>, {
		let mut operator = Operator('|');

		operator.get(tokens);
		// Dummy Error
		File::open(&"dummy").context(format!("Could find implicator: {}", "Line 4"))?;

		// implementation goes here
		todo!();
	}
}

impl FromToken for Operator {
	fn get<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<Child>
	where
		I: Iterator<Item = &'a Token>, {
		let mut parenthesis = Parenthesis('(');

		parenthesis.get(tokens);
		// Dummy Error
		File::open(&"dummy").context(format!("Could find implicator: {}", "Line 4"))?;

		// implementation goes here
		todo!();
	}
}

impl FromToken for Parenthesis {
	fn get<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<Child>
	where
		I: Iterator<Item = &'a Token>, {
		let mut identifier = Identifier('A');

		identifier.get(tokens);
		// Dummy Error
		File::open(&"dummy").context(format!("Could find implicator: {}", "Line 4"))?;

		// implementation goes here
		todo!();
	}
}

impl FromToken for Identifier {
	fn get<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<Child>
	where
		I: Iterator<Item = &'a Token>, {
		// Dummy Error
		File::open(&"dummy").context(format!("Could find implicator: {}", "Line 4"))?;

		// implementation goes here
		todo!();
	}
}

pub struct Node<'a> {
	token: &'a Token,
	left: Child<'a>,
	right: Child<'a>,
}



pub struct Parser {
	tokens: Vec<Token>,
}

impl<'a> Parser {
	pub fn new() -> Self {
		Parser { tokens: Vec::new() }
	}

	fn tokenize(&mut self, rule: &str) -> Result<Vec<Token>> {
		let mut lexer = rule.chars().peekable();
		let mut tokens: Vec<Token> = Vec::new();

		while let Some(c) = lexer.next() {
			match c {
				'(' | ')' => tokens.push(Token::Parenthesis(Parenthesis(c))),
				'!' | '+' | '|' | '^' => tokens.push(Token::Operator(Operator(c))),
				'A' ..= 'Z' => tokens.push(Token::Identifier(Identifier(c))),
				//TODO: Add Implicator
				c if c.is_whitespace() => {},
				_ => return Err(anyhow!("Unexpected character: {}", c))
			}
		}
		Ok(tokens)
	}

	pub fn parse(&mut self, rule: &str) -> Result<()> {
		let result = self.tokenize(rule).context(format!("Failed while lexing"))?;
		let mut tokens = result.iter().peekable();

		let mut consequent = Implicator;

		consequent.get(&mut tokens).context(format!("Could not parse: {}", "Line 4"))?;

		// implementation goes here
		todo!();
	}
}
