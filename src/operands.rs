#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    NOP,
    HALT,
    RET,
    ADD,
    LD,
    LDH
}
