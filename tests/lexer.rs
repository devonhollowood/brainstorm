extern crate brainstorm;
use brainstorm::lexer::*;

#[test]
fn no_whitespace() {
    let lexed = lex("+-<>,.[]");
    let types: Vec<TokenType> = lexed.iter().map(|t| t.token_type()).collect();
    let lines: Vec<LineNumber> = lexed.iter().map(|t| t.position().line()).collect();
    let cols: Vec<ColumnNumber> = lexed.iter().map(|t| t.position().column()).collect();
    assert_eq!(
        types,
        vec![TokenType::Incr, TokenType::Decr, TokenType::Left, TokenType::Right,
             TokenType::Read, TokenType::Write, TokenType::LoopOpen, TokenType::LoopClose
        ]
    );
    assert!(lines.iter().all(|&l| l == 0), "Not all lines equal zero");
    assert_eq!(
        cols,
        vec![0,1,2,3,4,5,6,7]
    );
}
