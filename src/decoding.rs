use crate::operands::Operand::{self, *};
use Register::*;
use bitcmp::{bitcmp, bitcap};


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Register {
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
pub enum RegOrImm {
    Reg(Register),
    Immediate(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arg {
    Direct(RegOrImm),
    Indirect(RegOrImm)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
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
        crate::decoding::Instruction::new($op, Vec::with_capacity(0))
    );
    ($op:expr, $($arg:expr),+) => (
        {
            let mut args = Vec::new();
            $(
                args.push($arg);
            )+
            crate::decoding::Instruction::new($op, args)
        }
    )
}

pub fn decode(inst: [u8; 3]) -> Instruction {
    use Arg::*;
    use RegOrImm::*;
    match inst {
        // LD r, r'
        [x, _, _] if bitcmp!(x, "01xxxyyy") => instruction!(LD, Direct(Reg(bitcap!(x, "00xxx000").into())), Direct(Reg(bitcap!(x, "00000yyy").into()))),
        // LD r, n
        [x, n, _] if bitcmp!(x, "00xxx110") => instruction!(LD, Direct(Reg(bitcap!(x, "00xxx000").into())), Direct(Immediate(n))),
        // LD r, (HL)
        [x, _, _] if bitcmp!(x, "01xxx110") => instruction!(LD, Direct(Reg(bitcap!(x, "00xxx000").into())), Indirect(Reg(HL))),
        // LD (HL), r
        [x, _, _] if bitcmp!(x, "01110xxx") => instruction!(LD, Indirect(Reg(HL)), Direct(Reg((bitcap!(x, "00000xxx")).into()))),
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
        [0xE2, _, _]   => instruction!(LDH, Indirect(Reg(C)), Direct(Reg(A))),
        // LDH A, (N)
        [0xF0, n, _]   => instruction!(LDH, Direct(Reg(n.into())), Indirect(Immediate(n.into()))),

        _ => panic!("invalid instruction {inst:#02X?}")
    }
}
