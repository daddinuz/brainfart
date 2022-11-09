#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Opcode {
    Left,
    Right,

    Decr,
    Incr,

    Read,
    Write,

    While,
    Until,

    Def,
    End,

    Ret,
    Call,
}
