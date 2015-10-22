use std::iter::Peekable;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Token {
    token_type: TokenType,
    position: Position
}

impl Token {
    pub fn token_type(&self) -> TokenType {
        self.token_type.clone()
    }

    pub fn position(&self) -> Position {
        self.position
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Position {
    line: LineNumber,
    column: ColumnNumber
}

pub type LineNumber = usize;
pub type ColumnNumber = usize;

impl Position {
    pub fn line(&self) -> LineNumber {
        self.line
    }

    pub fn column(&self) -> ColumnNumber {
        self.column
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TokenType {
    Incr,
    Decr,
    Left,
    Right,
    Read,
    Write,
    LoopOpen,
    LoopClose,
    Comment(String),
}

pub trait Tokenizeable: Iterator {
    fn tokenize(self) -> Tokens<Self> where
        Self: Iterator<Item=char> + Sized,
    {
        Tokens{iter: self.peekable(), position: Position{line: 0, column: 0}}
    }
}

impl<I: Iterator<Item=char>> Tokenizeable for I {}

pub fn lex(contents: &str) -> Vec<Token> {
    contents.chars().tokenize().collect()
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Tokens<I: Iterator<Item=char> + Sized> {
    iter: Peekable<I>,
    position: Position,
}

impl<I: Iterator<Item=char> + Sized> Iterator for Tokens<I> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let position = self.position; //save current position
        let mut token_type = match self.iter.next() {
            None => return None,
            Some('+') => TokenType::Incr,
            Some('-') => TokenType::Decr,
            Some('<') => TokenType::Left,
            Some('>') => TokenType::Right,
            Some(',') => TokenType::Read,
            Some('.') => TokenType::Write,
            Some('[') => TokenType::LoopOpen,
            Some(']') => TokenType::LoopClose,
            Some(c) => TokenType::Comment(format!("{}", c)),
        };
        self.position.column += 1;
        if let TokenType::Comment(mut s) = token_type {
            while let Some(&c) = self.iter.peek() {
                match c {
                    '+' | '-' | '<' | '>' | ',' | '.' | '[' | ']' => break,
                    '\n' => {
                        self.position.line += 1;
                        self.position.column = 0;
                        break;
                    },
                    _ => {
                        s.push(c);
                        self.iter.next().unwrap();
                    }
                }
            }
            s = s.trim().to_owned();
            if s.is_empty() {
                return self.next();
            }
            token_type = TokenType::Comment(s);
        }
        Some(Token {token_type: token_type, position: position})
    }
}
