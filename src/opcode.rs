#[derive(Debug, Copy, Clone)]
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
