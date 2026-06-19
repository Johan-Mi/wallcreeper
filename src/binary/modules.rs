use super::types::{ExternType, GlobalType, MemType, RecType, TableType, TagType, ValType};
use super::{instructions::Expr, values::Name};
use crate::Todo;
use alloc::vec::Vec;

#[derive(Clone, Copy)]
pub struct TypeIdx(pub u32);

pub struct FuncIdx(pub u32);

pub struct TableIdx(pub u32);

pub struct MemIdx(pub u32);

pub struct GlobalIdx(pub u32);

pub struct TagIdx(pub u32);

pub struct ElemIdx(pub u32);

pub struct DataIdx(pub u32);

pub struct LocalIdx(pub u32);

pub struct FieldIdx(pub u32);

pub struct LabelIdx(pub u32);

pub enum ExternIdx {
    Func(FuncIdx),
    Table(TableIdx),
    Memory(MemIdx),
    Global(GlobalIdx),
    Tag(TagIdx),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum SectionId {
    Custom,
    Type,
    Import,
    Function,
    Table,
    Memory,
    Tag,
    Global,
    Export,
    Start,
    Element,
    DataCount,
    Code,
    Data,
}

pub struct Custom {
    pub name: Name,
    pub bytes: Vec<u8>,
}

pub struct Import {
    pub module: Name,
    pub item: Name,
    pub r#type: ExternType,
}

pub struct Table {
    pub r#type: TableType,
    pub initializer: Expr,
}

pub struct Global {
    pub r#type: GlobalType,
    pub initializer: Expr,
}

pub struct Export {
    pub name: Name,
    pub definition: ExternIdx,
}

pub struct Elem(Todo);

pub struct Code {
    pub locals: Vec<ValType>,
    pub body: Expr,
}

pub struct Data {
    pub bytes: Vec<u8>,
    pub mode: DataMode,
}

pub enum DataMode {
    Active { memory: MemIdx, offset: Expr },
    Passive,
}

pub struct DataCnt(pub u32);

pub struct Module {
    pub customsecs: Vec<Custom>,
    pub typesec: Vec<RecType>,
    pub importsec: Vec<Import>,
    pub funcsec: Vec<TypeIdx>,
    pub tablesec: Vec<Table>,
    pub memsec: Vec<MemType>,
    pub tagsec: Vec<TagType>,
    pub globalsec: Vec<Global>,
    pub exportsec: Vec<Export>,
    pub startsec: Option<FuncIdx>,
    pub elemsec: Vec<Elem>,
    pub datacntsec: Option<u32>,
    pub codesec: Vec<Code>,
    pub datasec: Vec<Data>,
}
