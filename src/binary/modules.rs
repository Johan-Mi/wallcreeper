use crate::Todo;

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

pub enum SectionId {
    Custom,
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code,
    Data,
    DataCount,
    Tag,
}

pub struct Section<T>(T);

pub struct Custom(Todo);

pub struct Type(Todo);

pub struct Import(Todo);

pub struct Table(Todo);

pub struct Mem(Todo);

pub struct Global(Todo);

pub struct Export(Todo);

pub struct Start(Todo);

pub struct Elem(Todo);

pub struct Code(Todo);

pub struct Data(Todo);

pub struct DataCnt(Todo);

pub struct Tag(Todo);

pub struct Module(Todo);
