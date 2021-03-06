use super::lexer::{Token, TokenType, Position};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Instruction {
    Incr,
    Decr,
    Left,
    Right,
    Read,
    Write,
    Loop(Vec<Instruction>)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ParseError {
    UnmatchedLoopOpen(Position),
    UnmatchedLoopClose(Position),
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Instruction>, Vec<ParseError>> {
    let mut instructions = Vec::new();
    let mut errors = Vec::new();
    let mut loop_stack = Vec::new();
    for token in tokens {
        match token.token_type() {
            TokenType::Incr => instructions.push(Instruction::Incr),
            TokenType::Decr => instructions.push(Instruction::Decr),
            TokenType::Left => instructions.push(Instruction::Left),
            TokenType::Right => instructions.push(Instruction::Right),
            TokenType::Read => instructions.push(Instruction::Read),
            TokenType::Write => instructions.push(Instruction::Write),
            TokenType::LoopOpen => {
                loop_stack.push((instructions, token.position()));
                instructions = Vec::new();
            },
            TokenType::LoopClose => {
                match loop_stack.pop() {
                    Some((mut greater_instructions, _)) => {
                        greater_instructions.push(Instruction::Loop(instructions));
                        instructions = greater_instructions;
                    },
                    None => {
                        errors.push(ParseError::UnmatchedLoopClose(token.position()));
                    },
                }
            },
        }
    }
    errors.extend(loop_stack.iter().map(|&(_, pos)| ParseError::UnmatchedLoopOpen(pos)));
    if errors.is_empty() {
        Ok(instructions)
    }
    else{
        Err(errors)
    }
}
