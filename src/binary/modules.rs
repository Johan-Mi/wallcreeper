use super::types::{
    ExternType, GlobalType, MemType, RecType, RefType, TableType, TagType, ValType,
};
use super::{instructions::Expr, values::Name};
use alloc::vec::Vec;

#[derive(Clone, Copy)]
pub(crate) struct TypeIdx(pub u32);

pub(crate) struct FuncIdx(pub u32);

pub(crate) struct TableIdx(pub u32);

pub(crate) struct MemIdx(pub u32);

pub(crate) struct GlobalIdx(pub u32);

pub(crate) struct TagIdx(pub u32);

pub(crate) struct ElemIdx(pub u32);

pub(crate) struct DataIdx(pub u32);

pub(crate) struct LocalIdx(pub u32);

pub(crate) struct FieldIdx(pub u32);

pub(crate) struct LabelIdx(pub u32);

pub(crate) enum ExternIdx {
    Func(FuncIdx),
    Table(TableIdx),
    Memory(MemIdx),
    Global(GlobalIdx),
    Tag(TagIdx),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
pub(crate) enum SectionId {
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

pub(crate) struct Custom {
    pub name: Name,
    pub bytes: Vec<u8>,
}

pub(crate) struct Import {
    pub module: Name,
    pub item: Name,
    pub r#type: ExternType,
}

pub(crate) struct Table {
    pub r#type: TableType,
    pub initializer: Expr,
}

pub(crate) struct Global {
    pub r#type: GlobalType,
    pub initializer: Expr,
}

pub(crate) struct Export {
    pub name: Name,
    pub definition: ExternIdx,
}

pub(crate) struct Elem {
    pub r#type: RefType,
    pub items: Vec<Expr>,
    pub mode: ElemMode,
}

pub(crate) enum ElemMode {
    Active { table: TableIdx, offset: Expr },
    Passive,
    Declare,
}

pub(crate) struct Code {
    pub locals: Vec<ValType>,
    pub body: Expr,
}

pub(crate) struct Data {
    pub bytes: Vec<u8>,
    pub mode: DataMode,
}

pub(crate) enum DataMode {
    Active { memory: MemIdx, offset: Expr },
    Passive,
}

pub struct Module {
    pub(crate) customsecs: Vec<Custom>,
    pub(crate) typesec: Vec<RecType>,
    pub(crate) importsec: Vec<Import>,
    pub(crate) funcsec: Vec<TypeIdx>,
    pub(crate) tablesec: Vec<Table>,
    pub(crate) memsec: Vec<MemType>,
    pub(crate) tagsec: Vec<TagType>,
    pub(crate) globalsec: Vec<Global>,
    pub(crate) exportsec: Vec<Export>,
    pub(crate) startsec: Option<FuncIdx>,
    pub(crate) elemsec: Vec<Elem>,
    pub(crate) datacntsec: Option<u32>,
    pub(crate) codesec: Vec<Code>,
    pub(crate) datasec: Vec<Data>,
}
