#![no_std]

extern crate alloc;

pub mod binary {
    mod instructions;
    pub mod modules;
    pub mod parser;
    mod types;
    mod values;

    use instructions::{BlockType, CastOp, Catch, Expr, Instr, LaneIdx, MemArg, Nullable};
    use modules::{
        Code, Custom, Data, DataIdx, DataMode, Elem, ElemIdx, ElemMode, Export, ExternIdx,
        FieldIdx, FuncIdx, Global, GlobalIdx, Import, LabelIdx, LocalIdx, MemIdx, Module,
        SectionId, Table, TableIdx, TagIdx, TypeIdx,
    };
    use types::{
        AbsHeapType, AddressType, CompType, ExternType, FieldType, GlobalType, HeapType, Limits,
        MemType, Mut, NumType, PackType, RecType, RefType, ResultType, StorageType, SubType,
        TableType, TagType, ValType,
    };
    use values::Name;
}
