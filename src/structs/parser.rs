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
    fn get();
}

impl FromToken for Implicator {
    Operator::get();
    fn get() {
        println!("Implicator");
    }
}

impl FromToken for Operator {
    Parenthesis::get();
    fn get() {
        println!("Operator");
    }
}

impl FromToken for Parenthesis {
    Identifier::get();
    fn get() {
        println!("Parenthesis");
    }
}

impl FromToken for Identifier {
    fn get() {
        println!("Identifier");
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

    fn get_implicator<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Child>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Dummy Error
        File::open(&"dummy").context(format!("Could find implicator: {}", "Line 4"))?;

        // implementation goes here
        todo!();
    }

    fn get_operator<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Child>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Dummy Error
        File::open(&"dummy").context(format!("Could find operator: {}", "Line 4"))?;

        // implementation goes here
        todo!();
    }

    fn get_parenthesis<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Child>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Dummy Error
        File::open(&"dummy").context(format!("Could find parenthesis: {}", "Line 4"))?;

        // implementation goes here
        todo!();
    }

    fn get_identifier<I>(&mut self, tokens: &mut Peekable<I>) -> Result<Child>
    where
        I: Iterator<Item = &'a Token>,
    {
        // Dummy Error
        File::open(&"dummy").context(format!("Could find identifier: {}", "Line 4"))?;

        // implementation goes here
        todo!();
    }

    pub fn parse(&mut self) -> Result<()> {
        let tokens = Vec::new();
        let mut tokens = tokens.iter().peekable();

        self.get_implicator(&mut tokens)
            .context(format!("Could not parse: {}", "Line 4"))?;

        // implementation goes here
        todo!();
    }
}
