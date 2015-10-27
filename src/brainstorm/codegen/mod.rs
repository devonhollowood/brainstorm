use super::parser::Instruction;
use std::fmt::Write;

pub fn generate_asm<W>(asm: &mut W, instructions: &Vec<Instruction>) where
    W: Write,
{
    write_header(asm);
    for instruction in instructions {
        write_instruction(asm, 0, instruction);
    }
    write_footer(asm);
}

fn write_header<W>(mut asm: &mut W) where
    W: Write,
{
    write!(&mut asm, "section .text\n").unwrap();
    write!(&mut asm, "global {}\n", symbol::START).unwrap();
    write!(&mut asm, "{}:\n", symbol::START).unwrap();
}

fn write_instruction<W>(mut asm: &mut W, loop_num: usize, instruction: &Instruction) where
    W: Write,
{
    match *instruction {
        Instruction::Incr => {
            write!(&mut asm, "mov {}, [{}]\n", reg::B, symbol::OFFSET).unwrap();
            write!(&mut asm, "inc byte [{}+{}]\n", symbol::TABLE, reg::B).unwrap();
        },
        Instruction::Decr => {
            write!(&mut asm, "mov {}, [{}]\n", reg::B, symbol::OFFSET).unwrap();
            write!(&mut asm, "dec byte [{}+{}]\n", symbol::TABLE, reg::B).unwrap();
        },
        Instruction::Left => write!(&mut asm, "dec word [{}]\n", symbol::OFFSET).unwrap(),
        Instruction::Right => write!(&mut asm, "dec word [{}]\n", symbol::OFFSET).unwrap(),
        Instruction::Read => {
            write!(&mut asm, "mov {}, 1\n", reg::D).unwrap();
            write!(&mut asm, "mov {}, {}\n", reg::B, symbol::OFFSET).unwrap();
            write!(&mut asm, "mov {}, [{}+{}]\n", reg::C, symbol::TABLE, reg::B).unwrap();
            write!(&mut asm, "mov {}, 0\n", reg::B).unwrap();
            write!(&mut asm, "mov {}, 3\n", reg::A).unwrap();
            write!(&mut asm, "{}\n", symbol::SYSCALL).unwrap();
        },
        Instruction::Write => {
            write!(&mut asm, "mov {}, 1\n", reg::D).unwrap();
            write!(&mut asm, "mov {}, {}\n", reg::B, symbol::OFFSET).unwrap();
            write!(&mut asm, "mov {}, [{}+{}]\n", reg::C, symbol::TABLE, reg::B).unwrap();
            write!(&mut asm, "mov {}, 1\n", reg::B).unwrap();
            write!(&mut asm, "mov {}, 4\n", reg::A).unwrap();
            write!(&mut asm, "{}\n", symbol::SYSCALL).unwrap();
        },
        Instruction::Loop(ref loop_instrs) => {
            write!(&mut asm, "{}{}:\n", symbol::LOOP_LABEL, loop_num).unwrap();
            for loop_instr in loop_instrs {
                write_instruction(asm, loop_num + 1, &loop_instr);
            }
            write!(&mut asm, "mov {}, {}\n", reg::B, symbol::OFFSET).unwrap();
            write!(&mut asm, "cmp byte [{}+{}], 0\n", symbol::TABLE, reg::B).unwrap();
            write!(&mut asm, "je {}{}\n", symbol::LOOP_LABEL, loop_num).unwrap();
        }
    }
}

fn write_footer<W>(mut asm: &mut W) where
    W: Write,
{
    write!(&mut asm, "section .data\n").unwrap();
    write!(&mut asm, "{} db 0\n", symbol::OFFSET).unwrap();
    write!(&mut asm, "{} times 30000 db 0", symbol::TABLE).unwrap();
}

#[cfg(target_os = "linux")]
mod symbol {
     pub const TABLE: &'static str = "_table";
     pub const OFFSET: &'static str = "_offset";
     pub const LOOP_LABEL: &'static str = "loop";
     pub const SYSCALL: &'static str = "int 0x80";
     pub const START: &'static str = "_start";
}

#[cfg(target_pointer_width = "32")]
mod reg {
     pub const A: &'static str = "eax";
     pub const B: &'static str = "ebx";
     pub const C: &'static str = "ecx";
     pub const D: &'static str = "edx";
}

#[cfg(target_pointer_width = "64")]
mod reg {
     pub const A: &'static str = "rax";
     pub const B: &'static str = "rbx";
     pub const C: &'static str = "rcx";
     pub const D: &'static str = "rdx";
}
