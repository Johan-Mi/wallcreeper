use super::{instructions, modules, types, values};
use alloc::vec::Vec;

struct Error;

trait Parse: Sized {
    fn parse(input: &mut &[u8]) -> Result<Self, Error>;
}

impl<T: Parse> Parse for Vec<T> {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        let len: u32 = <_>::parse(input)?;
        let iter = core::iter::repeat_with(|| T::parse(input));
        iter.take(usize(len)).collect()
    }
}

const fn usize(n: u32) -> usize {
    const { assert!(size_of::<u32>() <= size_of::<usize>()) }
    n as usize
}

impl Parse for instructions::Instr {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        todo!()
    }
}

impl Parse for instructions::BlockType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        todo!()
    }
}

impl Parse for instructions::Catch {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(match u8::parse(input)? {
            0x00 => Self {
                tag: Some(<_>::parse(input)?),
                label: <_>::parse(input)?,
                by_ref: false,
            },
            0x01 => Self {
                tag: Some(<_>::parse(input)?),
                label: <_>::parse(input)?,
                by_ref: true,
            },
            0x02 => Self {
                tag: None,
                label: <_>::parse(input)?,
                by_ref: false,
            },
            0x03 => Self {
                tag: None,
                label: <_>::parse(input)?,
                by_ref: true,
            },
            _ => return Err(Error),
        })
    }
}

impl Parse for instructions::CastOp {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        use instructions::Nullable;
        let (source, target) = match u8::parse(input)? {
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

impl Parse for instructions::MemArg {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        use modules::MemIdx;
        let align: u32 = <_>::parse(input)?;
        let (memory, align) = if align < 1 << 6 {
            (MemIdx(0), align)
        } else if align < 1 << 7 {
            (<_>::parse(input)?, align - (1 << 7))
        } else {
            return Err(Error);
        };
        Ok(Self {
            memory,
            align,
            offset: <_>::parse(input)?,
        })
    }
}

impl Parse for instructions::LaneIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        <_>::parse(input).map(Self)
    }
}

impl Parse for instructions::Expr {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        let mut instrs = Vec::new();
        loop {
            match input.first() {
                Some(0x0b) => return Ok(Self(instrs)),
                None => return Err(Error),
                _ => instrs.push(<_>::parse(input)?),
            }
        }
    }
}

impl Parse for modules::TypeIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        <_>::parse(input).map(Self)
    }
}

impl Parse for modules::FuncIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        <_>::parse(input).map(Self)
    }
}

impl Parse for modules::TableIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        <_>::parse(input).map(Self)
    }
}

impl Parse for modules::MemIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        <_>::parse(input).map(Self)
    }
}

impl Parse for modules::GlobalIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        <_>::parse(input).map(Self)
    }
}

impl Parse for modules::TagIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        <_>::parse(input).map(Self)
    }
}

impl Parse for modules::ElemIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        <_>::parse(input).map(Self)
    }
}

impl Parse for modules::DataIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        <_>::parse(input).map(Self)
    }
}

impl Parse for modules::LocalIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        <_>::parse(input).map(Self)
    }
}

impl Parse for modules::FieldIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        <_>::parse(input).map(Self)
    }
}

impl Parse for modules::LabelIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        <_>::parse(input).map(Self)
    }
}

impl Parse for modules::ExternIdx {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        match u8::parse(input)? {
            0x00 => <_>::parse(input).map(Self::Func),
            0x01 => <_>::parse(input).map(Self::Table),
            0x02 => <_>::parse(input).map(Self::Memory),
            0x03 => <_>::parse(input).map(Self::Global),
            0x04 => <_>::parse(input).map(Self::Tag),
            _ => Err(Error),
        }
    }
}

impl Parse for modules::SectionId {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(match u8::parse(input)? {
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
        let name: values::Name = <_>::parse(input)?;
        let len_bytes = len_total.strict_sub(len_before.strict_sub(input.len()));
        let bytes = input.split_off(..len_bytes).ok_or(Error)?.into();
        Ok(Self { name, bytes })
    }
}

impl Parse for modules::Import {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(Self {
            module: <_>::parse(input)?,
            item: <_>::parse(input)?,
            r#type: <_>::parse(input)?,
        })
    }
}

impl Parse for modules::Table {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        todo!()
    }
}

impl Parse for modules::Global {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        todo!()
    }
}

impl Parse for modules::Export {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        let name = <_>::parse(input)?;
        let definition = <_>::parse(input)?;
        Ok(Self { name, definition })
    }
}

impl Parse for modules::Elem {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        todo!()
    }
}

impl Parse for modules::Code {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        todo!()
    }
}

impl Parse for modules::Data {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        todo!()
    }
}

impl Parse for modules::DataCnt {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        <_>::parse(input).map(Self)
    }
}

impl Parse for modules::Module {
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

            let section_id: SectionId = <_>::parse(input)?;
            if section_id != SectionId::Custom {
                if Some(section_id) <= section_id_prev {
                    return Err(Error);
                }
                section_id_prev = Some(section_id);
            }

            let len_wanted = usize(<_>::parse(input)?);
            let len_before = input.len();
            match section_id {
                SectionId::Custom => customsecs.push(modules::Custom::parse(len_wanted, input)?),
                SectionId::Type => typesec = <_>::parse(input)?,
                SectionId::Import => importsec = <_>::parse(input)?,
                SectionId::Function => funcsec = <_>::parse(input)?,
                SectionId::Table => tablesec = <_>::parse(input)?,
                SectionId::Memory => memsec = <_>::parse(input)?,
                SectionId::Tag => tagsec = <_>::parse(input)?,
                SectionId::Global => globalsec = <_>::parse(input)?,
                SectionId::Export => exportsec = <_>::parse(input)?,
                SectionId::Start => startsec = Some(<_>::parse(input)?),
                SectionId::Element => elemsec = <_>::parse(input)?,
                SectionId::DataCount => datacntsec = Some(<_>::parse(input)?),
                SectionId::Code => codesec = <_>::parse(input)?,
                SectionId::Data => datasec = <_>::parse(input)?,
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

impl Parse for types::NumType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(match u8::parse(input)? {
            0x7c => Self::F64,
            0x7d => Self::F32,
            0x7e => Self::I64,
            0x7f => Self::I32,
            _ => return Err(Error),
        })
    }
}

impl Parse for types::VecType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(match u8::parse(input)? {
            0x7b => Self::V128,
            _ => return Err(Error),
        })
    }
}

impl Parse for types::AbsHeapType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(match u8::parse(input)? {
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

impl Parse for types::HeapType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        <_>::parse(input).map(Self::Abstract).or_else(|_| todo!())
    }
}

impl Parse for types::RefType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        let (r#type, nullable) = if let Some(rest) = input.strip_prefix(&[0x63]) {
            *input = rest;
            (<_>::parse(input)?, true)
        } else if let Some(rest) = input.strip_prefix(&[0x64]) {
            *input = rest;
            (<_>::parse(input)?, false)
        } else {
            (types::HeapType::Abstract(<_>::parse(input)?), false)
        };
        Ok(Self {
            r#type,
            nullability: instructions::Nullable(nullable),
        })
    }
}

impl Parse for types::ValType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        (<_>::parse(input).map(Self::Num))
            .or_else(|_| <_>::parse(input).map(Self::Vec))
            .or_else(|_| <_>::parse(input).map(Self::Ref))
    }
}

impl Parse for types::ResultType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        <_>::parse(input).map(Self)
    }
}

impl Parse for types::Mut {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        u8::parse(input)?.try_into().map_err(|_| Error).map(Self)
    }
}

impl Parse for types::CompType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(match u8::parse(input)? {
            0x5e => Self::Array(<_>::parse(input)?),
            0x5f => Self::Struct(<_>::parse(input)?),
            0x60 => Self::Func {
                inputs: <_>::parse(input)?,
                outputs: <_>::parse(input)?,
            },
            _ => return Err(Error),
        })
    }
}

impl Parse for types::FieldType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        let r#type = <_>::parse(input)?;
        let mutability = <_>::parse(input).ok();
        Ok(Self { r#type, mutability })
    }
}

impl Parse for types::StorageType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        <_>::parse(input)
            .map(Self::Val)
            .or_else(|_| <_>::parse(input).map(Self::Pack))
    }
}

impl Parse for types::PackType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(match u8::parse(input)? {
            0x77 => Self::I16,
            0x78 => Self::I8,
            _ => return Err(Error),
        })
    }
}

impl Parse for types::RecType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        todo!()
    }
}

impl Parse for types::SubType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        todo!()
    }
}

impl Parse for types::Limits {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(match u8::parse(input)? {
            0x00 => Self {
                start: <_>::parse(input)?,
                end: None,
                address_type: types::AddressType::I32,
            },
            0x01 => Self {
                start: <_>::parse(input)?,
                end: Some(<_>::parse(input)?),
                address_type: types::AddressType::I32,
            },
            0x04 => Self {
                start: <_>::parse(input)?,
                end: None,
                address_type: types::AddressType::I64,
            },
            0x05 => Self {
                start: <_>::parse(input)?,
                end: Some(<_>::parse(input)?),
                address_type: types::AddressType::I64,
            },
            _ => return Err(Error),
        })
    }
}

impl Parse for types::TagType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        let 0x00 = u8::parse(input)? else {
            return Err(Error);
        };
        <_>::parse(input).map(Self)
    }
}

impl Parse for types::GlobalType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(Self {
            value_type: <_>::parse(input)?,
            mutability: <_>::parse(input).ok(),
        })
    }
}

impl Parse for types::MemType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        <_>::parse(input).map(Self)
    }
}

impl Parse for types::TableType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(Self {
            ref_type: <_>::parse(input)?,
            limits: <_>::parse(input)?,
        })
    }
}

impl Parse for types::ExternType {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        match u8::parse(input)? {
            0x00 => <_>::parse(input).map(Self::Func),
            0x01 => <_>::parse(input).map(Self::Table),
            0x02 => <_>::parse(input).map(Self::Mem),
            0x03 => <_>::parse(input).map(Self::Global),
            0x04 => <_>::parse(input).map(Self::Tag),
            _ => Err(Error),
        }
    }
}

impl<const N: usize> Parse for [u8; N] {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        Ok(input.split_off(..N).ok_or(Error)?.try_into().unwrap())
    }
}

impl Parse for u8 {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        input.split_off_first().copied().ok_or(Error)
    }
}

impl Parse for u32 {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        let mut n = 0;
        for shift in 0..Self::BITS / 7 {
            let byte: u8 = <_>::parse(input)?;
            n |= Self::from(byte & !(1 << 7)) << (shift * 7);
            if byte & (1 << 7) == 0 {
                return Ok(n);
            }
        }
        Err(Error)
    }
}

impl Parse for u64 {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        let mut n = 0;
        for shift in 0..Self::BITS / 7 {
            let byte: u8 = <_>::parse(input)?;
            n |= Self::from(byte & !(1 << 7)) << (shift * 7);
            if byte & (1 << 7) == 0 {
                return Ok(n);
            }
        }
        Err(Error)
    }
}

impl Parse for f32 {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        <[u8; _]>::parse(input).map(Self::from_le_bytes)
    }
}

impl Parse for f64 {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        <[u8; _]>::parse(input).map(Self::from_le_bytes)
    }
}

impl Parse for values::Name {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        let bytes: Vec<u8> = <_>::parse(input)?;
        bytes.try_into().map_err(|_| Error).map(values::Name)
    }
}
