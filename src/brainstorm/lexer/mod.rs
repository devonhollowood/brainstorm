#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Token {
    token_type: TokenType,
    position: Position
}

impl Token {
    pub fn new(token_type: TokenType, position: Position) -> Token {
        Token{token_type: token_type, position: position}
    }

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

impl Position {
    pub fn new(line: LineNumber, column: ColumnNumber) -> Position {
        Position {line: line, column: column}
    }

    pub fn line(&self) -> LineNumber {
        self.line
    }

    pub fn column(&self) -> ColumnNumber {
        self.column
    }
}

pub type LineNumber = usize;
pub type ColumnNumber = usize;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TokenType {
    Incr,
    Decr,
    Left,
    Right,
    Read,
    Write,
    LoopOpen,
    LoopClose,
}

pub trait Tokenizeable: Iterator {
    fn tokenize(self) -> Tokens<Self> where
        Self: Iterator<Item=char> + Sized,
    {
        Tokens{underlying: self, position: Position{line: 0, column: 0}}
    }
}

impl<I: Iterator<Item=char>> Tokenizeable for I {}

pub fn lex(contents: &str) -> Vec<Token> {
    contents.chars().tokenize().collect()
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Tokens<I: Iterator<Item=char> + Sized> {
    underlying: I,
    position: Position,
}

impl<I: Iterator<Item=char> + Sized> Iterator for Tokens<I> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        loop {
            let token_type = match self.underlying.next() {
                None => return None,
                Some('+') => TokenType::Incr,
                Some('-') => TokenType::Decr,
                Some('<') => TokenType::Left,
                Some('>') => TokenType::Right,
                Some(',') => TokenType::Read,
                Some('.') => TokenType::Write,
                Some('[') => TokenType::LoopOpen,
                Some(']') => TokenType::LoopClose,
                Some('\n') => {
                    self.position.line += 1;
                    self.position.column = 0;
                    continue;
                },
                Some(_) => {
                    self.position.column += 1;
                    continue;
                }
            };
            let position = self.position; //save position
            self.position.column += 1; //increment position
            return Some(Token{token_type: token_type, position: position});
        }
    }
}
