#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Left,
    Right,

    Decr,
    Incr,

    Read,
    Write,

    While,
    Repeat,

    Bind,
    Call,
    Ret,
}
