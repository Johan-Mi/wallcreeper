use super::types::{ExternType, RecType, TagType};
use super::values::Name;
use crate::Todo;
use alloc::vec::Vec;

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

#[derive(Clone, Copy, PartialEq, PartialOrd)]
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

pub struct Custom(Todo);

pub struct Import {
    module: Name,
    item: Name,
    r#type: ExternType,
}

pub struct Table(Todo);

pub struct Mem(Todo);

pub struct Global(Todo);

pub struct Export(Todo);

pub struct Elem(Todo);

pub struct Code(Todo);

pub struct Data(Todo);

pub struct DataCnt(pub u32);

pub struct Tag(Todo);

pub struct Module {
    pub customsecs: Vec<Custom>,
    pub typesec: Vec<RecType>,
    pub importsec: Vec<Import>,
    pub funcsec: Vec<TypeIdx>,
    pub tablesec: Vec<Table>,
    pub memsec: Vec<Mem>,
    pub tagsec: Vec<TagType>,
    pub globalsec: Vec<Global>,
    pub exportsec: Vec<Export>,
    pub startsec: Option<FuncIdx>,
    pub elemsec: Vec<Elem>,
    pub datacntsec: Option<u32>,
    pub codesec: Vec<Code>,
    pub datasec: Vec<Data>,
}
