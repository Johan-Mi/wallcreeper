use crate::Todo;
use crate::binary::modules::{LabelIdx, MemIdx, TagIdx, TypeIdx};
use crate::binary::types::ValType;
use alloc::vec::Vec;

pub struct Instr(Todo);

pub enum BlockType {
    None,
    Val(ValType),
    Idx(TypeIdx),
}

pub struct Catch {
    pub tag: Option<TagIdx>,
    pub label: LabelIdx,
    pub by_ref: bool,
}

pub struct CastOp {
    pub source: Nullable,
    pub target: Nullable,
}

#[derive(Clone, Copy)]
pub struct Nullable(pub bool);

pub struct MemArg {
    pub memory: MemIdx,
    pub align: u32,
    pub offset: u64,
}

pub struct LaneIdx(pub u8);

pub struct Expr(pub Vec<Instr>);
