extern crate brainstorm;
use brainstorm::lexer::{lex, Position};
use brainstorm::parser::*;

#[test]
fn simple() {
    let lexed = lex("+-<>,.");
    let parsed = parse(lexed);
    let expected = Ok(
        vec![
            Instruction::Incr,
            Instruction::Decr,
            Instruction::Left,
            Instruction::Right,
            Instruction::Read,
            Instruction::Write
        ]
    );
    assert_eq!(parsed, expected);
}

#[test]
fn empty_loop() {
    let lexed = lex("[]");
    let parsed = parse(lexed);
    let expected = Ok(
        vec![
            Instruction::Loop(vec![])
        ]
    );
    assert_eq!(parsed, expected);
}

#[test]
fn simple_loop() {
    let lexed = lex("[ - > + < ]");
    let parsed = parse(lexed);
    let expected = Ok(
        vec![
            Instruction::Loop(
                vec![
                    Instruction::Decr,
                    Instruction::Right,
                    Instruction::Incr,
                    Instruction::Left,
                ]
            )
        ]
    );
    assert_eq!(parsed, expected);
}

#[test]
fn compound_loop() {
    let lexed = lex("+ > [ [-] < - > ] < [ - > > . < < ]");
    let parsed = parse(lexed);
    let expected = Ok(
        vec![
            Instruction::Incr,
            Instruction::Right,
            Instruction::Loop(
                vec![
                    Instruction::Loop(
                        vec![
                            Instruction::Decr,
                        ]
                    ),
                    Instruction::Left,
                    Instruction::Decr,
                    Instruction::Right,
                ]
            ),
            Instruction::Left,
            Instruction::Loop(
                vec![
                    Instruction::Decr,
                    Instruction::Right,
                    Instruction::Right,
                    Instruction::Write,
                    Instruction::Left,
                    Instruction::Left,
                ]
            ),
        ]
    );
    assert_eq!(parsed, expected);
}

#[test]
fn unmatched_open() {
    let lexed = lex("+ > [ [ - < - > < [ - > > . < < ]");
    let parsed = parse(lexed);
    let expected = Err(
        vec![
            ParseError::UnmatchedLoopOpen(Position::new(0, 4)),
            ParseError::UnmatchedLoopOpen(Position::new(0, 6)),
        ]
    );
    assert_eq!(parsed, expected);
}

#[test]
fn unmatched_close() {
    let lexed = lex("+ > [ - ] < - > ] < - > > . < < ]");
    let parsed = parse(lexed);
    let expected = Err(
        vec![
            ParseError::UnmatchedLoopClose(Position::new(0, 16)),
            ParseError::UnmatchedLoopClose(Position::new(0, 32)),
        ]
    );
    assert_eq!(parsed, expected);
}

#[test]
fn unmatched_both() {
    let lexed = lex("+ > [ - ] < - > ] < [ - > > . < <");
    let parsed = parse(lexed);
    let expected = Err(
        vec![
            ParseError::UnmatchedLoopClose(Position::new(0, 16)),
            ParseError::UnmatchedLoopOpen(Position::new(0, 20)),
        ]
    );
    assert_eq!(parsed, expected);
}
