use crate::Todo;
use alloc::vec::Vec;

pub struct Instr(Todo);

pub struct BlockType(Todo);

pub struct Catch(Todo);

pub struct CastOp {
    pub source: Nullable,
    pub target: Nullable,
}

pub struct Nullable(pub bool);

pub struct MemArg(Todo);

pub struct LaneIdx(pub u8);

pub struct Expr(pub Vec<Instr>);
