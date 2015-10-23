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
    assert_eq!(cols, vec![0,1,2,3,4,5,6,7]);
}

#[test]
fn comment_only() {
    let lexed = lex(" A comment ");
    assert!(lexed.is_empty(), format!("lexed not empty: contains {:?}", lexed));
}

#[test]
fn mixed() {
    let lexed = lex("first , then .");
    let types: Vec<TokenType> = lexed.iter().map(|t| t.token_type()).collect();
    let lines: Vec<LineNumber> = lexed.iter().map(|t| t.position().line()).collect();
    let cols: Vec<ColumnNumber> = lexed.iter().map(|t| t.position().column()).collect();
    assert_eq!(types, vec![TokenType::Read, TokenType::Write]);
    assert!(lines.iter().all(|&l| l == 0), "not all lines equal zero");
    assert_eq!(cols, vec![6, 13]);
}

#[test]
fn newlines() {
    let input =
        [
            "",
            "    , read + and increment  ",
            "    < move back",
            "",
            ". . print twice"
        ].join("\n");
    let lexed = lex(&input);
    let types: Vec<TokenType> = lexed.iter().map(|t| t.token_type()).collect();
    let lines: Vec<LineNumber> = lexed.iter().map(|t| t.position().line()).collect();
    let cols: Vec<ColumnNumber> = lexed.iter().map(|t| t.position().column()).collect();
    let positions: Vec<_> = lines.into_iter().zip(cols.into_iter()).collect();
    assert_eq!(
        types,
        vec![TokenType::Read, TokenType::Incr, TokenType::Left, TokenType::Write,
             TokenType::Write
            ]
    );
    assert_eq!(positions,vec![(1, 4), (1, 11), (2, 4), (4, 0), (4, 2)]);
}
