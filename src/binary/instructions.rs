use crate::Todo;
use crate::binary::modules::{LabelIdx, TagIdx};
use alloc::vec::Vec;

pub struct Instr(Todo);

pub struct BlockType(Todo);

pub struct Catch {
    pub tag: Option<TagIdx>,
    pub label: LabelIdx,
    pub by_ref: bool,
}

pub struct CastOp {
    pub source: Nullable,
    pub target: Nullable,
}

pub struct Nullable(pub bool);

pub struct MemArg(Todo);

pub struct LaneIdx(pub u8);

pub struct Expr(pub Vec<Instr>);
