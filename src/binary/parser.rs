use super::{instructions, modules, types, values};
use alloc::vec::Vec;

struct Error;

impl instructions::Instr {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        todo!()
    }
}

impl instructions::BlockType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        if byte(0x40, input) {
            Ok(Self::None)
        } else {
            types::ValType::parse(input)
                .map(Self::Val)
                .or_else(|_| Ok(Self::Idx(modules::TypeIdx(leb128_s33_positive(input)?))))
        }
    }
}

impl instructions::Catch {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(match u8(input)? {
            0x00 => Self {
                tag: Some(modules::TagIdx::parse(input)?),
                label: modules::LabelIdx::parse(input)?,
                by_ref: false,
            },
            0x01 => Self {
                tag: Some(modules::TagIdx::parse(input)?),
                label: modules::LabelIdx::parse(input)?,
                by_ref: true,
            },
            0x02 => Self {
                tag: None,
                label: modules::LabelIdx::parse(input)?,
                by_ref: false,
            },
            0x03 => Self {
                tag: None,
                label: modules::LabelIdx::parse(input)?,
                by_ref: true,
            },
            _ => return Err(Error),
        })
    }
}

impl instructions::CastOp {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        use instructions::Nullable;
        let (source, target) = match u8(input)? {
            0x00 => (false, false),
            0x01 => (true, false),
            0x02 => (false, true),
            0x03 => (true, true),
            _ => return Err(Error),
        };
        let (source, target) = (Nullable(source), Nullable(target));
        Ok(Self { source, target })
    }
}

impl instructions::MemArg {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        use modules::MemIdx;
        let align = u32(input)?;
        let (memory, align) = if align < 1 << 6 {
            (MemIdx(0), align)
        } else if align < 1 << 7 {
            (MemIdx::parse(input)?, align - (1 << 7))
        } else {
            return Err(Error);
        };
        Ok(Self {
            memory,
            align,
            offset: u64(input)?,
        })
    }
}

impl instructions::LaneIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        u8(input).map(Self)
    }
}

impl instructions::Expr {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        let mut instrs = Vec::new();
        loop {
            match input.first() {
                Some(0x0b) => return Ok(Self(instrs)),
                None => return Err(Error),
                _ => instrs.push(instructions::Instr::parse(input)?),
            }
        }
    }
}

impl modules::TypeIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        u32(input).map(Self)
    }
}

impl modules::FuncIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        u32(input).map(Self)
    }
}

impl modules::TableIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        u32(input).map(Self)
    }
}

impl modules::MemIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        u32(input).map(Self)
    }
}

impl modules::GlobalIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        u32(input).map(Self)
    }
}

impl modules::TagIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        u32(input).map(Self)
    }
}

impl modules::ElemIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        u32(input).map(Self)
    }
}

impl modules::DataIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        u32(input).map(Self)
    }
}

impl modules::LocalIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        u32(input).map(Self)
    }
}

impl modules::FieldIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        u32(input).map(Self)
    }
}

impl modules::LabelIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        u32(input).map(Self)
    }
}

impl modules::ExternIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        match u8(input)? {
            0x00 => modules::FuncIdx::parse(input).map(Self::Func),
            0x01 => modules::TableIdx::parse(input).map(Self::Table),
            0x02 => modules::MemIdx::parse(input).map(Self::Memory),
            0x03 => modules::GlobalIdx::parse(input).map(Self::Global),
            0x04 => modules::TagIdx::parse(input).map(Self::Tag),
            _ => Err(Error),
        }
    }
}

impl modules::SectionId {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(match u8(input)? {
            0 => Self::Custom,
            1 => Self::Type,
            2 => Self::Import,
            3 => Self::Function,
            4 => Self::Table,
            5 => Self::Memory,
            6 => Self::Global,
            7 => Self::Export,
            8 => Self::Start,
            9 => Self::Element,
            10 => Self::Code,
            11 => Self::Data,
            12 => Self::DataCount,
            13 => Self::Tag,
            _ => return Err(Error),
        })
    }
}

impl modules::Custom {
    fn parse(len_total: usize, input: &mut &[u8]) -> Result<Self, Error> {
        let len_before = input.len();
        let name = values::Name::parse(input)?;
        let len_bytes = len_total.strict_sub(len_before.strict_sub(input.len()));
        let bytes = input.split_off(..len_bytes).ok_or(Error)?.into();
        Ok(Self { name, bytes })
    }
}

impl modules::Import {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(Self {
            module: values::Name::parse(input)?,
            item: values::Name::parse(input)?,
            r#type: types::ExternType::parse(input)?,
        })
    }
}

impl modules::Table {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        if byte(0x40, input) {
            if !byte(0x00, input) {
                return Err(Error);
            }
            Ok(Self {
                r#type: types::TableType::parse(input)?,
                initializer: instructions::Expr::parse(input)?,
            })
        } else {
            Ok(Self {
                r#type: types::TableType::parse(input)?,
                initializer: instructions::Expr([todo!()].into()),
            })
        }
    }
}

impl modules::Global {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(Self {
            r#type: types::GlobalType::parse(input)?,
            initializer: instructions::Expr::parse(input)?,
        })
    }
}

impl modules::Export {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        let name = values::Name::parse(input)?;
        let definition = modules::ExternIdx::parse(input)?;
        Ok(Self { name, definition })
    }
}

impl modules::Elem {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        let bit = {
            let bits = u32(input)?;
            let ..8 = bits else { return Err(Error) };
            move |i: u8| bits & (1 << i) != 0
        };
        todo!()
    }
}

impl modules::Code {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        struct Locals {
            count: u32,
            r#type: types::ValType,
        }

        impl Locals {
            fn parse(input: &mut &[u8]) -> Result<Self, Error> {
                let count = u32(input)?;
                let r#type = types::ValType::parse(input)?;
                Ok(Self { count, r#type })
            }
        }

        let len_wanted = usize(u32(input)?);
        let len_before = input.len();
        let locals = vec(Locals::parse, input)?;
        let body = instructions::Expr::parse(input)?;
        if len_wanted != len_before.strict_sub(input.len()) {
            return Err(Error);
        }
        let locals = locals
            .iter()
            .flat_map(|it| core::iter::repeat_n(it.r#type, usize(it.count)))
            .collect();
        Ok(Self { locals, body })
    }
}

impl modules::Data {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        let mode = match u32(input)? {
            0 => modules::DataMode::Active {
                memory: modules::MemIdx(0),
                offset: instructions::Expr::parse(input)?,
            },
            1 => modules::DataMode::Passive,
            2 => modules::DataMode::Active {
                memory: modules::MemIdx::parse(input)?,
                offset: instructions::Expr::parse(input)?,
            },
            _ => return Err(Error),
        };
        let bytes = vec(u8, input)?;
        Ok(Self { bytes, mode })
    }
}

impl modules::DataCnt {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        u32(input).map(Self)
    }
}

impl modules::Module {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        *input = input.strip_prefix(b"\0asm\x01\0\0\0").ok_or(Error)?;

        let mut customsecs = Vec::new();
        let mut typesec = Vec::new();
        let mut importsec = Vec::new();
        let mut funcsec = Vec::new();
        let mut tablesec = Vec::new();
        let mut memsec = Vec::new();
        let mut tagsec = Vec::new();
        let mut globalsec = Vec::new();
        let mut exportsec = Vec::new();
        let mut startsec = None;
        let mut elemsec = Vec::new();
        let mut datacntsec = None;
        let mut codesec = Vec::new();
        let mut datasec = Vec::new();

        let mut section_id_prev = None;
        while !input.is_empty() {
            use modules::SectionId;

            let section_id = SectionId::parse(input)?;
            if section_id != SectionId::Custom {
                if Some(section_id) <= section_id_prev {
                    return Err(Error);
                }
                section_id_prev = Some(section_id);
            }

            let len_wanted = usize(u32(input)?);
            let len_before = input.len();
            match section_id {
                SectionId::Custom => customsecs.push(modules::Custom::parse(len_wanted, input)?),
                SectionId::Type => typesec = vec(types::RecType::parse, input)?,
                SectionId::Import => importsec = vec(modules::Import::parse, input)?,
                SectionId::Function => funcsec = vec(modules::TypeIdx::parse, input)?,
                SectionId::Table => tablesec = vec(modules::Table::parse, input)?,
                SectionId::Memory => memsec = vec(types::MemType::parse, input)?,
                SectionId::Tag => tagsec = vec(types::TagType::parse, input)?,
                SectionId::Global => globalsec = vec(modules::Global::parse, input)?,
                SectionId::Export => exportsec = vec(modules::Export::parse, input)?,
                SectionId::Start => startsec = Some(modules::FuncIdx::parse(input)?),
                SectionId::Element => elemsec = vec(modules::Elem::parse, input)?,
                SectionId::DataCount => datacntsec = Some(u32(input)?),
                SectionId::Code => codesec = vec(modules::Code::parse, input)?,
                SectionId::Data => datasec = vec(modules::Data::parse, input)?,
            }
            if len_wanted != len_before.strict_sub(input.len()) {
                return Err(Error);
            }
        }

        Ok(Self {
            customsecs,
            typesec,
            importsec,
            funcsec,
            tablesec,
            memsec,
            tagsec,
            globalsec,
            exportsec,
            startsec,
            elemsec,
            datacntsec,
            codesec,
            datasec,
        })
    }
}

impl types::NumType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(match u8(input)? {
            0x7c => Self::F64,
            0x7d => Self::F32,
            0x7e => Self::I64,
            0x7f => Self::I32,
            _ => return Err(Error),
        })
    }
}

impl types::VecType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(match u8(input)? {
            0x7b => Self::V128,
            _ => return Err(Error),
        })
    }
}

impl types::AbsHeapType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(match u8(input)? {
            0x69 => Self::Exn,
            0x6a => Self::Array,
            0x6b => Self::Struct,
            0x6c => Self::I31,
            0x6d => Self::Eq,
            0x6e => Self::Any,
            0x6f => Self::Extern,
            0x70 => Self::Func,
            0x71 => Self::None,
            0x72 => Self::NoExtern,
            0x73 => Self::NoFunc,
            0x74 => Self::NoExn,
            _ => return Err(Error),
        })
    }
}

impl types::HeapType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        types::AbsHeapType::parse(input)
            .map(Self::Abstract)
            .or_else(|_| {
                Ok(Self::Concrete(modules::TypeIdx(leb128_s33_positive(
                    input,
                )?)))
            })
    }
}

impl types::RefType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        let (r#type, nullable) = if byte(0x63, input) {
            (types::HeapType::parse(input)?, true)
        } else if byte(0x64, input) {
            (types::HeapType::parse(input)?, false)
        } else {
            (
                types::HeapType::Abstract(types::AbsHeapType::parse(input)?),
                false,
            )
        };
        Ok(Self {
            r#type,
            nullability: instructions::Nullable(nullable),
        })
    }
}

impl types::ValType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        (types::NumType::parse(input).map(Self::Num))
            .or_else(|_| types::VecType::parse(input).map(Self::Vec))
            .or_else(|_| types::RefType::parse(input).map(Self::Ref))
    }
}

impl types::ResultType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        vec(types::ValType::parse, input).map(Self)
    }
}

impl types::Mut {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        u8(input)?.try_into().map_err(|_| Error).map(Self)
    }
}

impl types::CompType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(match u8(input)? {
            0x5e => Self::Array(types::FieldType::parse(input)?),
            0x5f => Self::Struct(vec(types::FieldType::parse, input)?),
            0x60 => Self::Func {
                inputs: types::ResultType::parse(input)?,
                outputs: types::ResultType::parse(input)?,
            },
            _ => return Err(Error),
        })
    }
}

impl types::FieldType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        let r#type = types::StorageType::parse(input)?;
        let mutability = types::Mut::parse(input).ok();
        Ok(Self { r#type, mutability })
    }
}

impl types::StorageType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        (types::ValType::parse(input).map(Self::Val))
            .or_else(|_| types::PackType::parse(input).map(Self::Pack))
    }
}

impl types::PackType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(match u8(input)? {
            0x77 => Self::I16,
            0x78 => Self::I8,
            _ => return Err(Error),
        })
    }
}

impl types::RecType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(Self(if byte(0x4e, input) {
            vec(types::SubType::parse, input)?
        } else {
            [types::SubType::parse(input)?].into()
        }))
    }
}

impl types::SubType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(if byte(0x4f, input) {
            Self {
                is_final: true,
                uses: vec(modules::TypeIdx::parse, input)?,
                comp: types::CompType::parse(input)?,
            }
        } else if byte(0x50, input) {
            Self {
                is_final: false,
                uses: vec(modules::TypeIdx::parse, input)?,
                comp: types::CompType::parse(input)?,
            }
        } else {
            Self {
                is_final: true,
                uses: Vec::new(),
                comp: types::CompType::parse(input)?,
            }
        })
    }
}

impl types::Limits {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(match u8(input)? {
            0x00 => Self {
                start: u64(input)?,
                end: None,
                address_type: types::AddressType::I32,
            },
            0x01 => Self {
                start: u64(input)?,
                end: Some(u64(input)?),
                address_type: types::AddressType::I32,
            },
            0x04 => Self {
                start: u64(input)?,
                end: None,
                address_type: types::AddressType::I64,
            },
            0x05 => Self {
                start: u64(input)?,
                end: Some(u64(input)?),
                address_type: types::AddressType::I64,
            },
            _ => return Err(Error),
        })
    }
}

impl types::TagType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        let 0x00 = u8(input)? else {
            return Err(Error);
        };
        modules::TypeIdx::parse(input).map(Self)
    }
}

impl types::GlobalType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(Self {
            value_type: types::ValType::parse(input)?,
            mutability: types::Mut::parse(input).ok(),
        })
    }
}

impl types::MemType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        types::Limits::parse(input).map(Self)
    }
}

impl types::TableType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(Self {
            ref_type: types::RefType::parse(input)?,
            limits: types::Limits::parse(input)?,
        })
    }
}

impl types::ExternType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        match u8(input)? {
            0x00 => modules::TypeIdx::parse(input).map(Self::Func),
            0x01 => types::TableType::parse(input).map(Self::Table),
            0x02 => types::MemType::parse(input).map(Self::Mem),
            0x03 => types::GlobalType::parse(input).map(Self::Global),
            0x04 => types::TagType::parse(input).map(Self::Tag),
            _ => Err(Error),
        }
    }
}

fn byte_array<const N: usize>(input: &mut &[u8]) -> Result<[u8; N], Error> {
    Ok(input.split_off(..N).ok_or(Error)?.try_into().unwrap())
}

fn u8(input: &mut &[u8]) -> Result<u8, Error> {
    input.split_off_first().copied().ok_or(Error)
}

fn uleb128<T>(input: &mut &[u8]) -> Result<T, Error>
where
    T: Default + From<u8> + core::ops::BitOrAssign + core::ops::Shl<usize, Output = T>,
{
    let mut n = T::default();
    for shift in 0..size_of::<T>() * 8 / 7 {
        let byte = u8(input)?;
        n |= T::from(byte & !(1 << 7)) << (shift * 7);
        if byte & (1 << 7) == 0 {
            return Ok(n);
        }
    }
    Err(Error)
}

fn leb128_s33_positive(input: &mut &[u8]) -> Result<u32, Error> {
    let mut n = 0;
    for shift in 0..33_usize.div_ceil(7) {
        let byte = u8(input)?;
        n |= i64::from(byte & !(1 << 7)) << (shift * 7);
        if byte & (1 << 7) == 0 {
            if byte & (1 << 6) != 0 {
                return Err(Error);
            }
            return n.try_into().map_err(|_| Error);
        }
    }
    Err(Error)
}

fn u32(input: &mut &[u8]) -> Result<u32, Error> {
    uleb128(input)
}

fn u64(input: &mut &[u8]) -> Result<u64, Error> {
    uleb128(input)
}

fn f32(input: &mut &[u8]) -> Result<f32, Error> {
    byte_array(input).map(f32::from_le_bytes)
}

fn f64(input: &mut &[u8]) -> Result<f64, Error> {
    byte_array(input).map(f64::from_le_bytes)
}

impl values::Name {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        let bytes: Vec<u8> = vec(u8, input)?;
        bytes.try_into().map_err(|_| Error).map(values::Name)
    }
}

fn byte(b: u8, input: &mut &[u8]) -> bool {
    input.strip_prefix(&[b]).inspect(|it| *input = it).is_some()
}

fn vec<T>(
    element: impl Fn(&mut &[u8]) -> Result<T, Error>,
    input: &mut &[u8],
) -> Result<Vec<T>, Error> {
    let len = u32(input)?;
    let iter = core::iter::repeat_with(|| element(input));
    iter.take(usize(len)).collect()
}

const fn usize(n: u32) -> usize {
    const { assert!(size_of::<u32>() <= size_of::<usize>()) }
    n as usize
}
