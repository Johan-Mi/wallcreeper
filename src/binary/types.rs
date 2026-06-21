use super::{Nullable, TypeIdx};
use alloc::vec::Vec;

#[derive(Clone, Copy)]
pub enum NumType {
    F64,
    F32,
    I64,
    I32,
    V128,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub enum HeapType {
    Abstract(AbsHeapType),
    Concrete(TypeIdx),
}

#[derive(Clone, Copy)]
pub struct RefType {
    pub r#type: HeapType,
    pub nullability: Nullable,
}

#[derive(Clone, Copy)]
pub enum ValType {
    Num(NumType),
    Ref(RefType),
}

pub struct ResultType(pub Vec<ValType>);

pub struct Mut(pub bool);

pub enum CompType {
    Array(FieldType),
    Struct(Vec<FieldType>),
    Func {
        inputs: ResultType,
        outputs: ResultType,
    },
}

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

pub struct RecType(pub Vec<SubType>);

pub struct SubType {
    pub is_final: bool,
    pub uses: Vec<TypeIdx>,
    pub comp: CompType,
}

pub struct Limits {
    pub start: u64,
    pub end: Option<u64>,
    pub address_type: AddressType,
}

pub enum AddressType {
    I32,
    I64,
}

pub struct TagType(pub TypeIdx);

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
    Func(TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
    Tag(TagType),
}
