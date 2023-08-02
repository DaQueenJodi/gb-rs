#[cfg(test)]
use crate::decoding::{decode, Register::*, RegOrImm::*, Arg::*};
use crate::instruction;
use crate::operands::Operand::*;

#[test]
fn decoding() {
    assert_eq!(decode([0x41, 0x0, 0x0]), instruction!(LD, Direct(Reg(B)), Direct(Reg(C))));
    assert_eq!(decode([0x06, 0xF, 0x0]), instruction!(LD, Direct(Reg(B)), Direct(Immediate(0xF))));
    assert_eq!(decode([0x46, 0x0, 0x0]), instruction!(LD, Direct(Reg(B)), Indirect(Reg(HL))));
    assert_eq!(decode([0x70, 0x0, 0x0]), instruction!(LD, Indirect(Reg(HL)), Direct(Reg(B))));
    assert_eq!(decode([0x36, 0xE, 0x0]), instruction!(LD, Indirect(Reg(HL)), Direct(Immediate(0xE))));
    assert_eq!(decode([0x0A, 0x0, 0x0]), instruction!(LD, Direct(Reg(A)), Indirect(Reg(BC))));
    assert_eq!(decode([0x1A, 0x0, 0x0]), instruction!(LD, Direct(Reg(A)), Indirect(Reg(DE))));
    assert_eq!(decode([0x02, 0x0, 0x0]), instruction!(LD, Indirect(Reg(BC)), Direct(Reg(A))));
    assert_eq!(decode([0x02, 0x0, 0x0]), instruction!(LD, Indirect(Reg(DE)), Direct(Reg(A))));
    assert_eq!(decode([0x0FA, 0xFF, 0xFF]), instruction!(LD, Direct(Reg(A)), Direct(Immediate(0xFF)), Direct(Immediate(0xFF))));
    assert_eq!(decode([0xEA, 0xFF, 0xFF]), instruction!(LD, Direct(Immediate(0xFF)), Direct(Immediate(0xFF)), Direct(Reg(A))));
}
