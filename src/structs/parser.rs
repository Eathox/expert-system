use anyhow::{Context, Result};
use expert_system::*;
use std::fs::File;
use std::iter::Peekable;

pub type Child<'a> = Box<Option<Node<'a>>>;

pub struct Implicator;
pub struct Operator(char);
pub struct Parenthesis(char);
pub struct Identifier(String);

pub enum Token {
	Implicator(Implicator),
	Operator(Operator),
	Parenthesis(Parenthesis),
	Identifier(Identifier), // Maybe just a char instead of String? --> A, B, C, D
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
		let mut identifier = Identifier(String::from("A"));

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

	fn tokenize(&mut self) -> Result<Vec<Token>> {
		// Dummy Error
		File::open(&"dummy").context(format!("Could not tokenize: {}", "Line 4"))?;

		// implementation goes here
		todo!();
	}

	pub fn parse(&mut self) -> Result<()> {
		let tokens = Vec::new();
		let mut tokens = tokens.iter().peekable();

		let mut consequent = Implicator;

		consequent.get(&mut tokens).context(format!("Could not parse: {}", "Line 4"))?;

		// implementation goes here
		todo!();
	}
}
