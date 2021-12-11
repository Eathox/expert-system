use anyhow::{anyhow, Context, Result};
use expert_system::*;
use std::fs::File;
use std::iter::Peekable;

pub type Child = Box<Option<Node>>;

#[derive(Copy, Clone)]
pub struct Implicator;
#[derive(Copy, Clone)]
pub struct Operator(char);
#[derive(Copy, Clone)]
pub struct Parenthesis(char);
#[derive(Copy, Clone)]
pub struct Identifier(char);

#[derive(Copy, Clone)]
pub enum Token {
	Implicator(Implicator),
	Operator(Operator),
	Parenthesis(Parenthesis),
	Identifier(Identifier),
}

pub trait FromToken {
	fn get<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<Child>
	where
		I: Iterator<Item = Token>;
}

impl FromToken for Implicator {
	fn get<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<Child>
	where
		I: Iterator<Item = Token>,
	{
		let mut operator = Operator('|');

		let lhs = operator.get(tokens);
		// Dummy Error
		File::open(&"dummy").context(format!("Could find implicator: {}", "Line 4"))?;

		match tokens.peek() {
			Some(Token::Operator(Operator('>'))) => {
				let token = tokens.next().unwrap();
				let mut operator = Operator('|');

				let rhs = operator.get(tokens);
				Ok(Box::new(Some(Node::new(token, lhs.unwrap(), rhs.unwrap()))))
			}
			_ => lhs,
		}
	}
}

impl FromToken for Operator {
	fn get<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<Child>
	where
		I: Iterator<Item = Token>,
	{
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
		I: Iterator<Item = Token>,
	{
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
		I: Iterator<Item = Token>,
	{
		// Dummy Error
		File::open(&"dummy").context(format!("Could find implicator: {}", "Line 4"))?;

		// implementation goes here
		todo!();
	}
}

pub struct Node {
	token: Token,
	left: Child,
	right: Child,
	// children: Vec<Node<'a>>
}

impl<'a> Node {
	pub fn new(token: Token, left: Child, right: Child) -> Self {
		Node {
			token,
			left: left,
			right: right,
		}
	}
}

// 	pub fn d_ast<'a, I>(&mut self,kens: &mut Peekable<I>) -> Result<Self>
// 	where
// 		I: Iterator<Item = Token>, {
// 		let mut antecedent = Implicator;
// 		let result = antecedent
// 			.get(&mut tokens)
// 			.context(format!("Could not parse: {}", "Line 4"))?;
// 		Ok((*result).unwrap())
// 	}
// }

pub struct Parser {
	tokens: Vec<Token>,
}

impl<'a> Parser {
	pub fn new() -> Self {
		Parser { tokens: Vec::new() }
	}

	pub fn tokenize(&mut self, rule: &str) -> Result<Vec<Token>> {
		let mut lexer = rule.chars().peekable();
		let mut tokens: Vec<Token> = Vec::new();

		while let Some(c) = lexer.next() {
			match c {
				'(' | ')' => tokens.push(Token::Parenthesis(Parenthesis(c))),
				'!' | '+' | '|' | '^' => tokens.push(Token::Operator(Operator(c))),
				'>' => tokens.push(Token::Implicator(Implicator)),
				'A'..='Z' => tokens.push(Token::Identifier(Identifier(c))),
				//TODO: Modify Implicator
				c if c.is_whitespace() => {}
				_ => return Err(anyhow!("Unexpected character: {}", c)),
			}
		}
		Ok(tokens)
	}

	pub fn parse(&mut self, tokens: &mut Vec<Token>) -> Result<()> {
		let mut tokens = tokens.iter().peekable();

		let mut consequent = Implicator;

		consequent
			.get(&mut tokens)
			.context(format!("Could not parse: {}", "Line 4"))?;

		// implementation goes here
		todo!();
	}
}
