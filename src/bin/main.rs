extern crate brainstorm;

fn main() {
    let prog = "\
    /* Hi Bassturds! */\
    +++++ +++++      loop 10x\
    [ >              enter loop\
    +++++ ++       > D = 68\
    +++++ +++++    > e = 101\
    +++++ +++++ ++ > v = 118\
    +++++ +++++ +  > o = 111\
    +++++ +++++ +  > n = 110\
    +++            > _ = 32\
    +++++ +++++ +  > r = 114\
    +++++ +++++ ++ > u = 117\
    +++++ +++++ +  > l = 108\
    +++++ +++++    > e = 101\
    +++++ +++++ +  > s = 115\
    +++              ! = 33\
    <<<<< <<<<< << - decrement counter\
    ]\
    >\
    --.    > D\
    +.     > e\
    --.    > v\
    +.     > o\
    .      > n\
    ++.    > _\
    ++++.  > r\
    ---.   > u\
    --.    > l\
    +.     > e\
    +++++. > s\
    +++.     !".to_owned();
    let lexed = brainstorm::lexer::lex(&prog);
    let parsed = brainstorm::parser::parse(lexed).unwrap();
    let mut output = String::new();
    brainstorm::codegen::generate_asm(&mut output, &parsed);
    println!("{}", output);
}
