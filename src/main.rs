mod operands;
use operands::Operand::{self, *};
use Register::*;

use bitmatch::bitmatch;


#[derive(Debug, Clone, PartialEq, Eq)]
enum Register {
    // 8 bit
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    F,
    // 16 bit
    AF,
    BC,
    DE,
    HL
}

impl std::convert::From<u8> for Register {
    fn from(val: u8) -> Register {
        match val {
            0 => Register::B,
            1 => Register::C,
            2 => Register::D,
            3 => Register::E,
            4 => Register::H,
            5 => Register::L,
            _ => panic!("invalid register number: {val}")

        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
enum RegOrImm {
    Reg(Register),
    Immediate(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Arg {
    direct(RegOrImm),
    Indirect(RegOrImm)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    operand: Operand,
    args: Vec<Arg>
}

impl Instruction {
    pub fn new(operand: Operand, args: Vec<Arg>) -> Self {
        Self {
            operand,
            args }
    }
}

#[macro_export]
macro_rules! instruction {
    ($op:expr) => (
        Instruction::new($op, Vec::with_capacity(0))
    );
    ($op:expr, $($arg:expr),+) => (
        {
            let mut args = Vec::new();
            $(
                args.push($arg);
            )+
            Instruction::new($op, args)
        }
    )
}

fn decode(inst: [u8; 3]) -> Instruction {
    use Arg::*;
    match inst {
        // LD r, r'
        [x, _, _] if x & 0b11000000 == 0b01000000 => instruction!(LD, Direct(Reg(Register::from(x  & 0b00111000))), Direct(Reg(Register::from(x & 0b00000111)))),
        // LD r, n
        [x, n, _] if x & 0b11000111 == 0b00000110 => instruction!(LD, Direct(Reg(Register::from(x  & 0b00111000))), Direct(Immediate(n))),
        // LD r, (HL)
        [x, _, _] if x & 0b11000111 == 0b01000110 => instruction!(LD, Direct(Reg(Register::from(x  & 0b00111000))), Indirect(Reg(HL))),
        // LD (HL), r
        [x, _, _] if x & 0b11111000 == 0b01110000 => instruction!(LD, Indirect(Reg(HL)), Reg(Register::from(x & 0b00000111))),
        // LD (HL), n
        [0x36, n, _]   => instruction!(LD, Indirect(Reg(HL)), Direct(Immediate(n))),
        // LD A, (BC)
        [0x0A, _, _]   => instruction!(LD, Direct(Reg(A)), Indirect(Reg(BC))),
        // LD A, (DE)
        [0x1A, _, _]   => instruction!(LD, Direct(Reg(A)), Indirect(Reg(DE))),
        // LD (BC), 
        [0x02, _, _]   => instruction!(LD, Indirect(Reg(BC)), Direct(Reg(A))),
        // LD (DE), A
        [0x12, _, _]   => instruction!(LD, Indirect(Reg(DE)), Direct(Reg(A))),
        // LD A, (nn)
        [0xFA, n1, n2] => instruction!(LD, Direct(Reg(A)), Direct(Immediate(n1)), Direct(Immediate(n2))),
        // LD (nn), A
        [0xEA, n1, n2] => instruction!(LD, Direct(Immediate(n1)), Direct(Immediate(n2)), Direct(Reg(A))),
        // LDH A, (C)
        [0xF2, _, _]   => instruction!(LDH, Direct(Reg(A)), Indirect(Reg(C))),
        // LDH (C), A
        [0xE2, _, _]   => instruction!(LDH, Indirect(C), Direct(Reg(A))),
        // LDH A, (N)
        [0xF0, n, _]   => instruction!(LDH, Direct(Reg(n)), Indirect(Immediate(n))),

        _ => panic!("invalid instruction {inst:#02X?}")
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn decoding() {
        assert_eq!(decode([0x41, 0x0, 0x0]), instruction!(LD, Arg::Reg(B), Arg::Reg(C)));
        assert_eq!(decode([0x80, 0x0, 0x0]), instruction!(ADD, Arg::Reg(B)));
    }
}



fn main() {
    todo!()
}
