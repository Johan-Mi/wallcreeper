use super::modules;
use crate::Todo;
use alloc::vec::Vec;

pub enum NumType {
    F64,
    F32,
    I64,
    I32,
}

pub enum VecType {
    V128,
}

pub enum AbsHeapType {
    Exn,
    Array,
    Struct,
    I31,
    Eq,
    Any,
    Extern,
    Func,
    None,
    NoExtern,
    NoFunc,
    NoExn,
}

pub struct HeapType(Todo);

pub struct RefType(Todo);

pub enum ValType {
    Num(NumType),
    Vec(VecType),
    Ref(RefType),
}

pub struct ResultType(Vec<ValType>);

pub struct Mut(pub bool);

pub struct CompType(Todo);

pub struct FieldType {
    pub r#type: StorageType,
    pub mutability: Option<Mut>,
}

pub enum StorageType {
    Val(ValType),
    Pack(PackType),
}

pub enum PackType {
    I16,
    I8,
}

pub struct RecType(Todo);

pub struct SubType(Todo);

pub struct Limits {
    pub start: u64,
    pub end: Option<u64>,
    pub address_type: AddressType,
}

pub enum AddressType {
    I32,
    I64,
}

pub struct TagType(pub modules::TypeIdx);

pub struct GlobalType {
    pub value_type: ValType,
    pub mutability: Option<Mut>,
}

pub struct MemType(pub Limits);

pub struct TableType {
    pub ref_type: RefType,
    pub limits: Limits,
}

pub enum ExternType {
    Func(modules::TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
    Tag(TagType),
}
