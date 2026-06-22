use super::{
    AbsHeapType, AddressType, BlockType, CastOp, Catch, Code, CompType, Custom, Data, DataIdx,
    DataMode, Elem, ElemIdx, ElemMode, Export, Expr, ExternIdx, ExternType, FieldIdx, FieldType,
    FuncIdx, Global, GlobalIdx, GlobalType, HeapType, Import, Instr, LabelIdx, LaneIdx, Limits,
    LocalIdx, MemArg, MemIdx, MemType, Module, Mut, Name, Nullable, NumType, PackType, RecType,
    RefType, ResultType, SectionId, StorageType, SubType, Table, TableIdx, TableType, TagIdx,
    TagType, TypeIdx, ValType,
};
use alloc::vec::Vec;

pub struct Error;

struct Parser<'src> {
    input: &'src [u8],
    stack: Vec<BlockKind>,
}

enum BlockKind {
    If,
    Other,
}

#[expect(clippy::too_many_lines, reason = "Bytecode parsing")]
impl Instr {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        fn flip<A, B, R>(f: impl Fn(A, B) -> R) -> impl Fn(B, A) -> R {
            move |b, a| f(a, b)
        }

        Ok(match u8(p)? {
            0x06..=0x07
            | 0x09
            | 0x16..=0x19
            | 0x1d..=0x1e
            | 0x27
            | 0xc5..=0xcf
            | 0xd7..=0xfa
            | 0xfe..=0xff => return Err(Error),
            0x00 => Self::Unreachable,
            0x01 => Self::Nop,
            0x02 => {
                p.stack.push(BlockKind::Other);
                Self::Block(BlockType::parse(p)?)
            }
            0x03 => {
                p.stack.push(BlockKind::Other);
                Self::Loop(BlockType::parse(p)?)
            }
            0x04 => {
                p.stack.push(BlockKind::If);
                Self::IfElse(BlockType::parse(p)?)
            }
            0x05 => p
                .stack
                .pop_if(|it| matches!(it, BlockKind::If))
                .map(|_| Self::Else)
                .ok_or(Error)?,
            0x08 => Self::Throw(TagIdx::parse(p)?),
            0x0a => Self::ThrowRef,
            0x0b => p.stack.pop().map(|_| Self::End).ok_or(Error)?,
            0x0c => Self::Br(LabelIdx::parse(p)?),
            0x0d => Self::BrIf(LabelIdx::parse(p)?),
            0x0e => Self::BrTable(vec(LabelIdx::parse, p)?, LabelIdx::parse(p)?),
            0x0f => Self::Return,
            0x10 => Self::Call(FuncIdx::parse(p)?),
            0x11 => flip(Self::CallIndirect)(TypeIdx::parse(p)?, TableIdx::parse(p)?),
            0x12 => Self::ReturnCall(FuncIdx::parse(p)?),
            0x13 => flip(Self::ReturnCallIndirect)(TypeIdx::parse(p)?, TableIdx::parse(p)?),
            0x14 => Self::CallRef(TypeIdx::parse(p)?),
            0x15 => Self::ReturnCallRef(TypeIdx::parse(p)?),
            0x1a => Self::Drop,
            0x1b => Self::Select(Vec::new()),
            0x1c => Self::Select(vec(ValType::parse, p)?),
            0x1f => {
                p.stack.push(BlockKind::Other);
                Self::TryTable(BlockType::parse(p)?, vec(Catch::parse, p)?)
            }
            0x20 => Self::Local·Get(LocalIdx::parse(p)?),
            0x21 => Self::Local·Set(LocalIdx::parse(p)?),
            0x22 => Self::Local·Tee(LocalIdx::parse(p)?),
            0x23 => Self::Global·Get(GlobalIdx::parse(p)?),
            0x24 => Self::Global·Set(GlobalIdx::parse(p)?),
            0x25 => Self::Table·Get(TableIdx::parse(p)?),
            0x26 => Self::Table·Set(TableIdx::parse(p)?),
            0x28 => Self::I32·Load(MemArg::parse(p)?),
            0x29 => Self::I64·Load(MemArg::parse(p)?),
            0x2a => Self::F32·Load(MemArg::parse(p)?),
            0x2b => Self::F64·Load(MemArg::parse(p)?),
            0x2c => Self::I32·Load8S(MemArg::parse(p)?),
            0x2d => Self::I32·Load8U(MemArg::parse(p)?),
            0x2e => Self::I32·Load16S(MemArg::parse(p)?),
            0x2f => Self::I32·Load16U(MemArg::parse(p)?),
            0x30 => Self::I64·Load8S(MemArg::parse(p)?),
            0x31 => Self::I64·Load8U(MemArg::parse(p)?),
            0x32 => Self::I64·Load16S(MemArg::parse(p)?),
            0x33 => Self::I64·Load16U(MemArg::parse(p)?),
            0x34 => Self::I64·Load32S(MemArg::parse(p)?),
            0x35 => Self::I64·Load32U(MemArg::parse(p)?),
            0x36 => Self::I32·Store(MemArg::parse(p)?),
            0x37 => Self::I64·Store(MemArg::parse(p)?),
            0x38 => Self::F32·Store(MemArg::parse(p)?),
            0x39 => Self::F64·Store(MemArg::parse(p)?),
            0x3a => Self::I32·Store8(MemArg::parse(p)?),
            0x3b => Self::I32·Store16(MemArg::parse(p)?),
            0x3c => Self::I64·Store8(MemArg::parse(p)?),
            0x3d => Self::I64·Store16(MemArg::parse(p)?),
            0x3e => Self::I64·Store32(MemArg::parse(p)?),
            0x3f => Self::Memory·Size(MemIdx::parse(p)?),
            0x40 => Self::Memory·Grow(MemIdx::parse(p)?),
            0x41 => Self::I32·Const(i32::from_le_bytes(byte_array(p)?)),
            0x42 => Self::I64·Const(i64::from_le_bytes(byte_array(p)?)),
            0x43 => Self::F32·Const(f32::from_le_bytes(byte_array(p)?)),
            0x44 => Self::F64·Const(f64::from_le_bytes(byte_array(p)?)),
            0x45 => Self::I32·Eqz,
            0x46 => Self::I32·Eq,
            0x47 => Self::I32·Ne,
            0x48 => Self::I32·LtS,
            0x49 => Self::I32·LtU,
            0x4a => Self::I32·GtS,
            0x4b => Self::I32·GtU,
            0x4c => Self::I32·LeS,
            0x4d => Self::I32·LeU,
            0x4e => Self::I32·GeS,
            0x4f => Self::I32·GeU,
            0x50 => Self::I64·Eqz,
            0x51 => Self::I64·Eq,
            0x52 => Self::I64·Ne,
            0x53 => Self::I64·LtS,
            0x54 => Self::I64·LtU,
            0x55 => Self::I64·GtS,
            0x56 => Self::I64·GtU,
            0x57 => Self::I64·LeS,
            0x58 => Self::I64·LeU,
            0x59 => Self::I64·GeS,
            0x5a => Self::I64·GeU,
            0x5b => Self::F32·Eq,
            0x5c => Self::F32·Ne,
            0x5d => Self::F32·Lt,
            0x5e => Self::F32·Gt,
            0x5f => Self::F32·Le,
            0x60 => Self::F32·Ge,
            0x61 => Self::F64·Eq,
            0x62 => Self::F64·Ne,
            0x63 => Self::F64·Lt,
            0x64 => Self::F64·Gt,
            0x65 => Self::F64·Le,
            0x66 => Self::F64·Ge,
            0x67 => Self::I32·Clz,
            0x68 => Self::I32·Ctz,
            0x69 => Self::I32·Popcnt,
            0x6a => Self::I32·Add,
            0x6b => Self::I32·Sub,
            0x6c => Self::I32·Mul,
            0x6d => Self::I32·DivS,
            0x6e => Self::I32·DivU,
            0x6f => Self::I32·RemS,
            0x70 => Self::I32·RemU,
            0x71 => Self::I32·And,
            0x72 => Self::I32·Or,
            0x73 => Self::I32·Xor,
            0x74 => Self::I32·Shl,
            0x75 => Self::I32·ShrS,
            0x76 => Self::I32·ShrU,
            0x77 => Self::I32·Rotl,
            0x78 => Self::I32·Rotr,
            0x79 => Self::I64·Clz,
            0x7a => Self::I64·Ctz,
            0x7b => Self::I64·Popcnt,
            0x7c => Self::I64·Add,
            0x7d => Self::I64·Sub,
            0x7e => Self::I64·Mul,
            0x7f => Self::I64·DivS,
            0x80 => Self::I64·DivU,
            0x81 => Self::I64·RemS,
            0x82 => Self::I64·RemU,
            0x83 => Self::I64·And,
            0x84 => Self::I64·Or,
            0x85 => Self::I64·Xor,
            0x86 => Self::I64·Shl,
            0x87 => Self::I64·ShrS,
            0x88 => Self::I64·ShrU,
            0x89 => Self::I64·Rotl,
            0x8a => Self::I64·Rotr,
            0x8b => Self::F32·Abs,
            0x8c => Self::F32·Neg,
            0x8d => Self::F32·Ceil,
            0x8e => Self::F32·Floor,
            0x8f => Self::F32·Trunc,
            0x90 => Self::F32·Nearest,
            0x91 => Self::F32·Sqrt,
            0x92 => Self::F32·Add,
            0x93 => Self::F32·Sub,
            0x94 => Self::F32·Mul,
            0x95 => Self::F32·Div,
            0x96 => Self::F32·Min,
            0x97 => Self::F32·Max,
            0x98 => Self::F32·Copysign,
            0x99 => Self::F64·Abs,
            0x9a => Self::F64·Neg,
            0x9b => Self::F64·Ceil,
            0x9c => Self::F64·Floor,
            0x9d => Self::F64·Trunc,
            0x9e => Self::F64·Nearest,
            0x9f => Self::F64·Sqrt,
            0xa0 => Self::F64·Add,
            0xa1 => Self::F64·Sub,
            0xa2 => Self::F64·Mul,
            0xa3 => Self::F64·Div,
            0xa4 => Self::F64·Min,
            0xa5 => Self::F64·Max,
            0xa6 => Self::F64·Copysign,
            0xa7 => Self::I32·WrapI64,
            0xa8 => Self::I32·TruncSF32,
            0xa9 => Self::I32·TruncUF32,
            0xaa => Self::I32·TruncSF64,
            0xab => Self::I32·TruncUF64,
            0xac => Self::I64·ExtendSI32,
            0xad => Self::I64·ExtendUI32,
            0xae => Self::I64·TruncSF32,
            0xaf => Self::I64·TruncUF32,
            0xb0 => Self::I64·TruncSF64,
            0xb1 => Self::I64·TruncUF64,
            0xb2 => Self::F32·ConvertSI32,
            0xb3 => Self::F32·ConvertUI32,
            0xb4 => Self::F32·ConvertSI64,
            0xb5 => Self::F32·ConvertUI64,
            0xb6 => Self::F32·DemoteF64,
            0xb7 => Self::F64·ConvertSI32,
            0xb8 => Self::F64·ConvertUI32,
            0xb9 => Self::F64·ConvertSI64,
            0xba => Self::F64·ConvertUI64,
            0xbb => Self::F64·PromoteF32,
            0xbc => Self::I32·ReinterpretF32,
            0xbd => Self::I64·ReinterpretF64,
            0xbe => Self::F32·ReinterpretI32,
            0xbf => Self::F64·ReinterpretI64,
            0xc0 => Self::I32·Extend8S,
            0xc1 => Self::I32·Extend16S,
            0xc2 => Self::I64·Extend8S,
            0xc3 => Self::I64·Extend16S,
            0xc4 => Self::I64·Extend32S,
            0xd0 => Self::Ref·Null(HeapType::parse(p)?),
            0xd1 => Self::Ref·IsNull,
            0xd2 => Self::Ref·Func(FuncIdx::parse(p)?),
            0xd3 => Self::Ref·Eq,
            0xd4 => Self::Ref·AsNonNull,
            0xd5 => Self::BrOnNull(LabelIdx::parse(p)?),
            0xd6 => Self::BrOnNonNull(LabelIdx::parse(p)?),
            0xfb => Self::aggregate(p)?,
            0xfc => match u32(p)? {
                0 => Self::I32·TruncSatSF32,
                1 => Self::I32·TruncSatUF32,
                2 => Self::I32·TruncSatSF64,
                3 => Self::I32·TruncSatUF64,
                4 => Self::I64·TruncSatSF32,
                5 => Self::I64·TruncSatUF32,
                6 => Self::I64·TruncSatSF64,
                7 => Self::I64·TruncSatUF64,
                8 => flip(Self::Memory·Init)(DataIdx::parse(p)?, MemIdx::parse(p)?),
                9 => Self::Data·Drop(DataIdx::parse(p)?),
                10 => Self::Memory·Copy(MemIdx::parse(p)?, MemIdx::parse(p)?),
                11 => Self::Memory·Fill(MemIdx::parse(p)?),
                12 => flip(Self::Table·Init)(ElemIdx::parse(p)?, TableIdx::parse(p)?),
                13 => Self::Elem·Drop(ElemIdx::parse(p)?),
                14 => Self::Table·Copy(TableIdx::parse(p)?, TableIdx::parse(p)?),
                15 => Self::Table·Grow(TableIdx::parse(p)?),
                16 => Self::Table·Size(TableIdx::parse(p)?),
                17 => Self::Table·Fill(TableIdx::parse(p)?),
                _ => return Err(Error),
            },
            0xfd => Self::vector(p)?,
        })
    }

    fn aggregate(p: &mut Parser) -> Result<Self, Error> {
        Ok(match u32(p)? {
            0 => Self::Struct·New(TypeIdx::parse(p)?),
            1 => Self::Struct·NewDefault(TypeIdx::parse(p)?),
            2 => Self::Struct·Get(TypeIdx::parse(p)?, FieldIdx::parse(p)?),
            3 => Self::Struct·GetS(TypeIdx::parse(p)?, FieldIdx::parse(p)?),
            4 => Self::Struct·GetU(TypeIdx::parse(p)?, FieldIdx::parse(p)?),
            5 => Self::Struct·Set(TypeIdx::parse(p)?, FieldIdx::parse(p)?),
            6 => Self::Array·New(TypeIdx::parse(p)?),
            7 => Self::Array·NewDefault(TypeIdx::parse(p)?),
            8 => Self::Array·NewFixed(TypeIdx::parse(p)?, u32(p)?),
            9 => Self::Array·NewData(TypeIdx::parse(p)?, DataIdx::parse(p)?),
            10 => Self::Array·NewElem(TypeIdx::parse(p)?, ElemIdx::parse(p)?),
            11 => Self::Array·Get(TypeIdx::parse(p)?),
            12 => Self::Array·GetS(TypeIdx::parse(p)?),
            13 => Self::Array·GetU(TypeIdx::parse(p)?),
            14 => Self::Array·Set(TypeIdx::parse(p)?),
            15 => Self::Array·Len,
            16 => Self::Array·Fill(TypeIdx::parse(p)?),
            17 => Self::Array·Copy(TypeIdx::parse(p)?, TypeIdx::parse(p)?),
            18 => Self::Array·InitData(TypeIdx::parse(p)?, DataIdx::parse(p)?),
            19 => Self::Array·InitElem(TypeIdx::parse(p)?, ElemIdx::parse(p)?),
            20 => Self::Ref·Test(RefType {
                r#type: HeapType::parse(p)?,
                nullability: Nullable(false),
            }),
            21 => Self::Ref·Test(RefType {
                r#type: HeapType::parse(p)?,
                nullability: Nullable(true),
            }),
            22 => Self::Ref·Cast(RefType {
                r#type: HeapType::parse(p)?,
                nullability: Nullable(false),
            }),
            23 => Self::Ref·Cast(RefType {
                r#type: HeapType::parse(p)?,
                nullability: Nullable(true),
            }),
            24 => {
                let castop = CastOp::parse(p)?;
                let source = RefType {
                    r#type: HeapType::parse(p)?,
                    nullability: castop.source,
                };
                let target = RefType {
                    r#type: HeapType::parse(p)?,
                    nullability: castop.target,
                };
                Self::BrOnCast(LabelIdx::parse(p)?, source, target)
            }
            25 => {
                let castop = CastOp::parse(p)?;
                let source = RefType {
                    r#type: HeapType::parse(p)?,
                    nullability: castop.source,
                };
                let target = RefType {
                    r#type: HeapType::parse(p)?,
                    nullability: castop.target,
                };
                Self::BrOnCastFail(LabelIdx::parse(p)?, source, target)
            }
            26 => Self::Any·ConvertExtern,
            27 => Self::Extern·ConvertAny,
            28 => Self::Ref·I31,
            29 => Self::I31·GetS,
            30 => Self::I31·GetU,
            _ => return Err(Error),
        })
    }

    fn vector(p: &mut Parser) -> Result<Self, Error> {
        Ok(match u32(p)? {
            0 => Self::V128·Load(MemArg::parse(p)?),
            1 => Self::V128·Load8x8S(MemArg::parse(p)?),
            2 => Self::V128·Load8x8U(MemArg::parse(p)?),
            3 => Self::V128·Load16x4S(MemArg::parse(p)?),
            4 => Self::V128·Load16x4U(MemArg::parse(p)?),
            5 => Self::V128·Load32x2S(MemArg::parse(p)?),
            6 => Self::V128·Load32x2U(MemArg::parse(p)?),
            7 => Self::V128·Load8Splat(MemArg::parse(p)?),
            8 => Self::V128·Load16Splat(MemArg::parse(p)?),
            9 => Self::V128·Load32Splat(MemArg::parse(p)?),
            10 => Self::V128·Load64Splat(MemArg::parse(p)?),
            11 => Self::V128·Store(MemArg::parse(p)?),
            12 => Self::V128·Const(u128::from_le_bytes(byte_array(p)?)),
            13 => Self::I8x16·Shuffle(byte_array(p)?.map(LaneIdx)),
            14 => Self::I8x16·Swizzle,
            15 => Self::I8x16·Splat,
            16 => Self::I16x8·Splat,
            17 => Self::I32x4·Splat,
            18 => Self::I64x2·Splat,
            19 => Self::F32x4·Splat,
            20 => Self::F64x2·Splat,
            21 => Self::I8x16·ExtractLaneS(LaneIdx::parse(p)?),
            22 => Self::I8x16·ExtractLaneU(LaneIdx::parse(p)?),
            23 => Self::I8x16·ReplaceLane(LaneIdx::parse(p)?),
            24 => Self::I16x8·ExtractLaneS(LaneIdx::parse(p)?),
            25 => Self::I16x8·ExtractLaneU(LaneIdx::parse(p)?),
            26 => Self::I16x8·ReplaceLane(LaneIdx::parse(p)?),
            27 => Self::I32x4·ExtractLane(LaneIdx::parse(p)?),
            28 => Self::I32x4·ReplaceLane(LaneIdx::parse(p)?),
            29 => Self::I64x2·ExtractLane(LaneIdx::parse(p)?),
            30 => Self::I64x2·ReplaceLane(LaneIdx::parse(p)?),
            31 => Self::F32x4·ExtractLane(LaneIdx::parse(p)?),
            32 => Self::F32x4·ReplaceLane(LaneIdx::parse(p)?),
            33 => Self::F64x2·ExtractLane(LaneIdx::parse(p)?),
            34 => Self::F64x2·ReplaceLane(LaneIdx::parse(p)?),
            35 => Self::I8x16·Eq,
            36 => Self::I8x16·Ne,
            37 => Self::I8x16·LtS,
            38 => Self::I8x16·LtU,
            39 => Self::I8x16·GtS,
            40 => Self::I8x16·GtU,
            41 => Self::I8x16·LeS,
            42 => Self::I8x16·LeU,
            43 => Self::I8x16·GeS,
            44 => Self::I8x16·GeU,
            45 => Self::I16x8·Eq,
            46 => Self::I16x8·Ne,
            47 => Self::I16x8·LtS,
            48 => Self::I16x8·LtU,
            49 => Self::I16x8·GtS,
            50 => Self::I16x8·GtU,
            51 => Self::I16x8·LeS,
            52 => Self::I16x8·LeU,
            53 => Self::I16x8·GeS,
            54 => Self::I16x8·GeU,
            55 => Self::I32x4·Eq,
            56 => Self::I32x4·Ne,
            57 => Self::I32x4·LtS,
            58 => Self::I32x4·LtU,
            59 => Self::I32x4·GtS,
            60 => Self::I32x4·GtU,
            61 => Self::I32x4·LeS,
            62 => Self::I32x4·LeU,
            63 => Self::I32x4·GeS,
            64 => Self::I32x4·GeU,
            65 => Self::F32x4·Eq,
            66 => Self::F32x4·Ne,
            67 => Self::F32x4·Lt,
            68 => Self::F32x4·Gt,
            69 => Self::F32x4·Le,
            70 => Self::F32x4·Ge,
            71 => Self::F64x2·Eq,
            72 => Self::F64x2·Ne,
            73 => Self::F64x2·Lt,
            74 => Self::F64x2·Gt,
            75 => Self::F64x2·Le,
            76 => Self::F64x2·Ge,
            77 => Self::V128·Not,
            78 => Self::V128·And,
            79 => Self::V128·Andnot,
            80 => Self::V128·Or,
            81 => Self::V128·Xor,
            82 => Self::V128·Bitselect,
            83 => Self::V128·AnyTrue,
            84 => Self::V128·Load8Lane(MemArg::parse(p)?, LaneIdx::parse(p)?),
            85 => Self::V128·Load16Lane(MemArg::parse(p)?, LaneIdx::parse(p)?),
            86 => Self::V128·Load32Lane(MemArg::parse(p)?, LaneIdx::parse(p)?),
            87 => Self::V128·Load64Lane(MemArg::parse(p)?, LaneIdx::parse(p)?),
            88 => Self::V128·Store8Lane(MemArg::parse(p)?, LaneIdx::parse(p)?),
            89 => Self::V128·Store16Lane(MemArg::parse(p)?, LaneIdx::parse(p)?),
            90 => Self::V128·Store32Lane(MemArg::parse(p)?, LaneIdx::parse(p)?),
            91 => Self::V128·Store64Lane(MemArg::parse(p)?, LaneIdx::parse(p)?),
            92 => Self::V128·Load32Zero(MemArg::parse(p)?),
            93 => Self::V128·Load64Zero(MemArg::parse(p)?),
            94 => Self::F32x4·DemoteZeroF64x2,
            95 => Self::F64x2·PromoteLowF32x4,
            96 => Self::I8x16·Abs,
            97 => Self::I8x16·Neg,
            98 => Self::I8x16·Popcnt,
            99 => Self::I8x16·AllTrue,
            100 => Self::I8x16·Bitmask,
            101 => Self::I8x16·NarrowI16x8S,
            102 => Self::I8x16·NarrowI16x8U,
            103 => Self::F32x4·Ceil,
            104 => Self::F32x4·Floor,
            105 => Self::F32x4·Trunc,
            106 => Self::F32x4·Nearest,
            107 => Self::I8x16·Shl,
            108 => Self::I8x16·ShrS,
            109 => Self::I8x16·ShrU,
            110 => Self::I8x16·Add,
            111 => Self::I8x16·AddSatS,
            112 => Self::I8x16·AddSatU,
            113 => Self::I8x16·Sub,
            114 => Self::I8x16·SubSatS,
            115 => Self::I8x16·SubSatU,
            116 => Self::F64x2·Ceil,
            117 => Self::F64x2·Floor,
            118 => Self::I8x16·MinS,
            119 => Self::I8x16·MinU,
            120 => Self::I8x16·MaxS,
            121 => Self::I8x16·MaxU,
            122 => Self::F64x2·Trunc,
            123 => Self::I8x16·AvgrU,
            124 => Self::I16x8·ExtaddPairwiseSI8x16,
            125 => Self::I16x8·ExtaddPairwiseUI8x16,
            126 => Self::I32x4·ExtaddPairwiseSI16x8,
            127 => Self::I32x4·ExtaddPairwiseUI16x8,
            128 => Self::I16x8·Abs,
            129 => Self::I16x8·Neg,
            130 => Self::I16x8·Q15MulrSatS,
            131 => Self::I16x8·AllTrue,
            132 => Self::I16x8·Bitmask,
            133 => Self::I16x8·NarrowI32x4S,
            134 => Self::I16x8·NarrowI32x4U,
            135 => Self::I16x8·ExtendLowSI8x16,
            136 => Self::I16x8·ExtendHighSI8x16,
            137 => Self::I16x8·ExtendLowUI8x16,
            138 => Self::I16x8·ExtendHighUI8x16,
            139 => Self::I16x8·Shl,
            140 => Self::I16x8·ShrS,
            141 => Self::I16x8·ShrU,
            142 => Self::I16x8·Add,
            143 => Self::I16x8·AddSatS,
            144 => Self::I16x8·AddSatU,
            145 => Self::I16x8·Sub,
            146 => Self::I16x8·SubSatS,
            147 => Self::I16x8·SubSatU,
            148 => Self::F64x2·Nearest,
            149 => Self::I16x8·Mul,
            150 => Self::I16x8·MinS,
            151 => Self::I16x8·MinU,
            152 => Self::I16x8·MaxS,
            153 => Self::I16x8·MaxU,
            155 => Self::I16x8·AvgrU,
            156 => Self::I16x8·ExtmulLowSI8x16,
            157 => Self::I16x8·ExtmulHighSI8x16,
            158 => Self::I16x8·ExtmulLowUI8x16,
            159 => Self::I16x8·ExtmulHighUI8x16,
            160 => Self::I32x4·Abs,
            161 => Self::I32x4·Neg,
            163 => Self::I32x4·AllTrue,
            164 => Self::I32x4·Bitmask,
            167 => Self::I32x4·ExtendLowSI16x8,
            168 => Self::I32x4·ExtendHighSI16x8,
            169 => Self::I32x4·ExtendLowUI16x8,
            170 => Self::I32x4·ExtendHighUI16x8,
            171 => Self::I32x4·Shl,
            172 => Self::I32x4·ShrS,
            173 => Self::I32x4·ShrU,
            174 => Self::I32x4·Add,
            177 => Self::I32x4·Sub,
            181 => Self::I32x4·Mul,
            182 => Self::I32x4·MinS,
            183 => Self::I32x4·MinU,
            184 => Self::I32x4·MaxS,
            185 => Self::I32x4·MaxU,
            186 => Self::I32x4·DotSI16x8,
            188 => Self::I32x4·ExtmulLowSI16x8,
            189 => Self::I32x4·ExtmulHighSI16x8,
            190 => Self::I32x4·ExtmulLowUI16x8,
            191 => Self::I32x4·ExtmulHighUI16x8,
            192 => Self::I64x2·Abs,
            193 => Self::I64x2·Neg,
            195 => Self::I64x2·AllTrue,
            196 => Self::I64x2·Bitmask,
            199 => Self::I64x2·ExtendLowSI32x4,
            200 => Self::I64x2·ExtendHighSI32x4,
            201 => Self::I64x2·ExtendLowUI32x4,
            202 => Self::I64x2·ExtendHighUI32x4,
            203 => Self::I64x2·Shl,
            204 => Self::I64x2·ShrS,
            205 => Self::I64x2·ShrU,
            206 => Self::I64x2·Add,
            209 => Self::I64x2·Sub,
            213 => Self::I64x2·Mul,
            214 => Self::I64x2·Eq,
            215 => Self::I64x2·Ne,
            216 => Self::I64x2·LtS,
            217 => Self::I64x2·GtS,
            218 => Self::I64x2·LeS,
            219 => Self::I64x2·GeS,
            220 => Self::I64x2·ExtmulLowSI32x4,
            221 => Self::I64x2·ExtmulHighSI32x4,
            222 => Self::I64x2·ExtmulLowUI32x4,
            223 => Self::I64x2·ExtmulHighUI32x4,
            224 => Self::F32x4·Abs,
            225 => Self::F32x4·Neg,
            227 => Self::F32x4·Sqrt,
            228 => Self::F32x4·Add,
            229 => Self::F32x4·Sub,
            230 => Self::F32x4·Mul,
            231 => Self::F32x4·Div,
            232 => Self::F32x4·Min,
            233 => Self::F32x4·Max,
            234 => Self::F32x4·Pmin,
            235 => Self::F32x4·Pmax,
            236 => Self::F64x2·Abs,
            237 => Self::F64x2·Neg,
            239 => Self::F64x2·Sqrt,
            240 => Self::F64x2·Add,
            241 => Self::F64x2·Sub,
            242 => Self::F64x2·Mul,
            243 => Self::F64x2·Div,
            244 => Self::F64x2·Min,
            245 => Self::F64x2·Max,
            246 => Self::F64x2·Pmin,
            247 => Self::F64x2·Pmax,
            248 => Self::I32x4·TruncSatSF32x4,
            249 => Self::I32x4·TruncSatUF32x4,
            250 => Self::F32x4·ConvertSI32x4,
            251 => Self::F32x4·ConvertUI32x4,
            252 => Self::I32x4·TruncSatSZeroF64x2,
            253 => Self::I32x4·TruncSatUZeroF64x2,
            254 => Self::F64x2·ConvertLowSI32x4,
            255 => Self::F64x2·ConvertLowUI32x4,
            256 => Self::I8x16·RelaxedSwizzle,
            257 => Self::I32x4·RelaxedTruncSF32x4,
            258 => Self::I32x4·RelaxedTruncUF32x4,
            259 => Self::I32x4·RelaxedTruncSZeroF64x2,
            260 => Self::I32x4·RelaxedTruncUZeroF64x2,
            261 => Self::F32x4·RelaxedMadd,
            262 => Self::F32x4·RelaxedNmadd,
            263 => Self::F64x2·RelaxedMadd,
            264 => Self::F64x2·RelaxedNmadd,
            265 => Self::I8x16·RelaxedLaneselect,
            266 => Self::I16x8·RelaxedLaneselect,
            267 => Self::I32x4·RelaxedLaneselect,
            268 => Self::I64x2·RelaxedLaneselect,
            269 => Self::F32x4·RelaxedMin,
            270 => Self::F32x4·RelaxedMax,
            271 => Self::F64x2·RelaxedMin,
            272 => Self::F64x2·RelaxedMax,
            273 => Self::I16x8·RelaxedQ15MulrS,
            274 => Self::I16x8·RelaxedDotSI8x16,
            275 => Self::I32x4·RelaxedDotAddSI16x8,
            _ => return Err(Error),
        })
    }
}

impl BlockType {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        if byte(0x40, p) {
            Ok(Self::None)
        } else {
            ValType::parse(p)
                .map(Self::Val)
                .or_else(|_| Ok(Self::Idx(TypeIdx(leb128_s33_positive(p)?))))
        }
    }
}

impl Catch {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        Ok(match u8(p)? {
            0x00 => Self {
                tag: Some(TagIdx::parse(p)?),
                label: LabelIdx::parse(p)?,
                by_ref: false,
            },
            0x01 => Self {
                tag: Some(TagIdx::parse(p)?),
                label: LabelIdx::parse(p)?,
                by_ref: true,
            },
            0x02 => Self {
                tag: None,
                label: LabelIdx::parse(p)?,
                by_ref: false,
            },
            0x03 => Self {
                tag: None,
                label: LabelIdx::parse(p)?,
                by_ref: true,
            },
            _ => return Err(Error),
        })
    }
}

impl CastOp {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        let (source, target) = match u8(p)? {
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

impl MemArg {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        let align = u32(p)?;
        let true = align < 1 << 7 else {
            return Err(Error);
        };
        let (memory, align) = if let Some(align) = align.checked_sub(1 << 6) {
            (MemIdx::parse(p)?, align)
        } else {
            (MemIdx(0), align)
        };
        Ok(Self {
            memory,
            align,
            offset: u64(p)?,
        })
    }
}

impl LaneIdx {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        u8(p).map(Self)
    }
}

impl Expr {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        p.stack.push(BlockKind::Other);
        let here = p.stack.len();
        let mut instrs = Vec::new();
        loop {
            let instr = Instr::parse(p)?;
            let is_end = matches!(instr, Instr::End);
            instrs.push(instr);
            if is_end {
                if p.stack.pop().is_none() {
                    return Err(Error);
                }
                if p.stack.len() == here {
                    return Ok(Self(instrs));
                }
            }
        }
    }
}

impl TypeIdx {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        u32(p).map(Self)
    }
}

impl FuncIdx {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        u32(p).map(Self)
    }
}

impl TableIdx {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        u32(p).map(Self)
    }
}

impl MemIdx {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        u32(p).map(Self)
    }
}

impl GlobalIdx {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        u32(p).map(Self)
    }
}

impl TagIdx {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        u32(p).map(Self)
    }
}

impl ElemIdx {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        u32(p).map(Self)
    }
}

impl DataIdx {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        u32(p).map(Self)
    }
}

impl LocalIdx {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        u32(p).map(Self)
    }
}

impl FieldIdx {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        u32(p).map(Self)
    }
}

impl LabelIdx {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        u32(p).map(Self)
    }
}

impl ExternIdx {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        match u8(p)? {
            0x00 => FuncIdx::parse(p).map(Self::Func),
            0x01 => TableIdx::parse(p).map(Self::Table),
            0x02 => MemIdx::parse(p).map(Self::Memory),
            0x03 => GlobalIdx::parse(p).map(Self::Global),
            0x04 => TagIdx::parse(p).map(Self::Tag),
            _ => Err(Error),
        }
    }
}

impl SectionId {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        Ok(match u8(p)? {
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

impl Custom {
    fn parse(len_total: usize, p: &mut Parser) -> Result<Self, Error> {
        let len_before = p.input.len();
        let name = Name::parse(p)?;
        let len_bytes = len_total.strict_sub(len_before.strict_sub(p.input.len()));
        let bytes = p.input.split_off(..len_bytes).ok_or(Error)?.into();
        Ok(Self { name, bytes })
    }
}

impl Import {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        Ok(Self {
            module: Name::parse(p)?,
            item: Name::parse(p)?,
            r#type: ExternType::parse(p)?,
        })
    }
}

impl Table {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        if byte(0x40, p) {
            if !byte(0x00, p) {
                return Err(Error);
            }
            Ok(Self {
                r#type: TableType::parse(p)?,
                initializer: Expr::parse(p)?,
            })
        } else {
            let r#type = TableType::parse(p)?;
            let initializer = Expr([Instr::Ref·Null(r#type.ref_type.r#type)].into());
            Ok(Self {
                r#type,
                initializer,
            })
        }
    }
}

impl Global {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        Ok(Self {
            r#type: GlobalType::parse(p)?,
            initializer: Expr::parse(p)?,
        })
    }
}

impl Export {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        let name = Name::parse(p)?;
        let definition = ExternIdx::parse(p)?;
        Ok(Self { name, definition })
    }
}

impl Elem {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        let elemkind = |p| {
            if byte(0x00, p) {
                Ok(RefType {
                    r#type: HeapType::Abstract(AbsHeapType::Func),
                    nullability: Nullable(false),
                })
            } else {
                Err(Error)
            }
        };

        Ok(match u32(p)? {
            0 => {
                let offset = Expr::parse(p)?;
                let items = vec(FuncIdx::parse, p)?
                    .into_iter()
                    .map(|it| Expr([Instr::Ref·Func(it)].into()))
                    .collect();
                Self {
                    r#type: RefType {
                        r#type: HeapType::Abstract(AbsHeapType::Func),
                        nullability: Nullable(false),
                    },
                    items,
                    mode: ElemMode::Active {
                        table: TableIdx(0),
                        offset,
                    },
                }
            }
            bits @ (1 | 3) => {
                let r#type = elemkind(p)?;
                let items = vec(FuncIdx::parse, p)?
                    .into_iter()
                    .map(|it| Expr([Instr::Ref·Func(it)].into()))
                    .collect();
                Self {
                    r#type,
                    items,
                    mode: if bits & 0b10 == 0 {
                        ElemMode::Passive
                    } else {
                        ElemMode::Declare
                    },
                }
            }
            2 => {
                let table = TableIdx::parse(p)?;
                let offset = Expr::parse(p)?;
                let r#type = elemkind(p)?;
                let items = vec(FuncIdx::parse, p)?
                    .into_iter()
                    .map(|it| Expr([Instr::Ref·Func(it)].into()))
                    .collect();
                Self {
                    r#type,
                    items,
                    mode: ElemMode::Active { table, offset },
                }
            }
            4 => {
                let offset = Expr::parse(p)?;
                let items = vec(Expr::parse, p)?;
                Self {
                    r#type: RefType {
                        r#type: HeapType::Abstract(AbsHeapType::Func),
                        nullability: Nullable(true),
                    },
                    items,
                    mode: ElemMode::Active {
                        table: TableIdx(0),
                        offset,
                    },
                }
            }
            bits @ (5 | 7) => Self {
                r#type: elemkind(p)?,
                items: vec(Expr::parse, p)?,
                mode: if bits & 0b10 == 0 {
                    ElemMode::Passive
                } else {
                    ElemMode::Declare
                },
            },
            6 => {
                let table = TableIdx::parse(p)?;
                let offset = Expr::parse(p)?;
                let r#type = elemkind(p)?;
                let items = vec(Expr::parse, p)?;
                Self {
                    r#type,
                    items,
                    mode: ElemMode::Active { table, offset },
                }
            }
            _ => return Err(Error),
        })
    }
}

impl Code {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        struct Locals {
            count: u32,
            r#type: ValType,
        }

        impl Locals {
            fn parse(p: &mut Parser) -> Result<Self, Error> {
                let count = u32(p)?;
                let r#type = ValType::parse(p)?;
                Ok(Self { count, r#type })
            }
        }

        let len_wanted = usize(u32(p)?);
        let len_before = p.input.len();
        let locals = vec(Locals::parse, p)?;
        let body = Expr::parse(p)?;
        if len_wanted != len_before.strict_sub(p.input.len()) {
            return Err(Error);
        }
        let locals = locals
            .iter()
            .flat_map(|it| core::iter::repeat_n(it.r#type, usize(it.count)))
            .collect();
        Ok(Self { locals, body })
    }
}

impl Data {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        let mode = match u32(p)? {
            0 => DataMode::Active {
                memory: MemIdx(0),
                offset: Expr::parse(p)?,
            },
            1 => DataMode::Passive,
            2 => DataMode::Active {
                memory: MemIdx::parse(p)?,
                offset: Expr::parse(p)?,
            },
            _ => return Err(Error),
        };
        let bytes = vec(u8, p)?;
        Ok(Self { bytes, mode })
    }
}

impl Module {
    /// # Errors
    ///
    /// This function will return an error if the p is syntactically invalid.
    pub fn parse(input: &[u8]) -> Result<Self, Error> {
        let input = input.strip_prefix(b"\0asm\x01\0\0\0").ok_or(Error)?;
        let stack = Vec::new();
        let mut p = Parser { input, stack };

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
        while !p.input.is_empty() {
            let section_id = SectionId::parse(&mut p)?;
            if section_id != SectionId::Custom {
                if Some(section_id) <= section_id_prev {
                    return Err(Error);
                }
                section_id_prev = Some(section_id);
            }

            let len_wanted = usize(u32(&mut p)?);
            let len_before = p.input.len();
            match section_id {
                SectionId::Custom => {
                    customsecs.push(Custom::parse(len_wanted, &mut p)?);
                }
                SectionId::Type => typesec = vec(RecType::parse, &mut p)?,
                SectionId::Import => importsec = vec(Import::parse, &mut p)?,
                SectionId::Function => funcsec = vec(TypeIdx::parse, &mut p)?,
                SectionId::Table => tablesec = vec(Table::parse, &mut p)?,
                SectionId::Memory => memsec = vec(MemType::parse, &mut p)?,
                SectionId::Tag => tagsec = vec(TagType::parse, &mut p)?,
                SectionId::Global => globalsec = vec(Global::parse, &mut p)?,
                SectionId::Export => exportsec = vec(Export::parse, &mut p)?,
                SectionId::Start => startsec = Some(FuncIdx::parse(&mut p)?),
                SectionId::Element => elemsec = vec(Elem::parse, &mut p)?,
                SectionId::DataCount => datacntsec = Some(u32(&mut p)?),
                SectionId::Code => codesec = vec(Code::parse, &mut p)?,
                SectionId::Data => datasec = vec(Data::parse, &mut p)?,
            }
            if len_wanted != len_before.strict_sub(p.input.len()) {
                return Err(Error);
            }
        }

        if !p.input.is_empty() {
            return Err(Error);
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

impl NumType {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        Ok(match u8(p)? {
            0x7b => Self::V128,
            0x7c => Self::F64,
            0x7d => Self::F32,
            0x7e => Self::I64,
            0x7f => Self::I32,
            _ => return Err(Error),
        })
    }
}

impl AbsHeapType {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        Ok(match u8(p)? {
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

impl HeapType {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        AbsHeapType::parse(p)
            .map(Self::Abstract)
            .or_else(|_| Ok(Self::Concrete(TypeIdx(leb128_s33_positive(p)?))))
    }
}

impl RefType {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        let (r#type, nullable) = if byte(0x63, p) {
            (HeapType::parse(p)?, true)
        } else if byte(0x64, p) {
            (HeapType::parse(p)?, false)
        } else {
            (HeapType::Abstract(AbsHeapType::parse(p)?), false)
        };
        Ok(Self {
            r#type,
            nullability: Nullable(nullable),
        })
    }
}

impl ValType {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        (NumType::parse(p).map(Self::Num)).or_else(|_| RefType::parse(p).map(Self::Ref))
    }
}

impl ResultType {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        vec(ValType::parse, p).map(Self)
    }
}

impl Mut {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        u8(p)?.try_into().map_err(|_| Error).map(Self)
    }
}

impl CompType {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        Ok(match u8(p)? {
            0x5e => Self::Array(FieldType::parse(p)?),
            0x5f => Self::Struct(vec(FieldType::parse, p)?),
            0x60 => Self::Func {
                inputs: ResultType::parse(p)?,
                outputs: ResultType::parse(p)?,
            },
            _ => return Err(Error),
        })
    }
}

impl FieldType {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        let r#type = StorageType::parse(p)?;
        let mutability = Mut::parse(p).ok();
        Ok(Self { r#type, mutability })
    }
}

impl StorageType {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        (ValType::parse(p).map(Self::Val)).or_else(|_| PackType::parse(p).map(Self::Pack))
    }
}

impl PackType {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        Ok(match u8(p)? {
            0x77 => Self::I16,
            0x78 => Self::I8,
            _ => return Err(Error),
        })
    }
}

impl RecType {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        Ok(Self(if byte(0x4e, p) {
            vec(SubType::parse, p)?
        } else {
            [SubType::parse(p)?].into()
        }))
    }
}

impl SubType {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        Ok(if byte(0x4f, p) {
            Self {
                is_final: true,
                uses: vec(TypeIdx::parse, p)?,
                comp: CompType::parse(p)?,
            }
        } else if byte(0x50, p) {
            Self {
                is_final: false,
                uses: vec(TypeIdx::parse, p)?,
                comp: CompType::parse(p)?,
            }
        } else {
            Self {
                is_final: true,
                uses: Vec::new(),
                comp: CompType::parse(p)?,
            }
        })
    }
}

impl Limits {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        Ok(match u8(p)? {
            0x00 => Self {
                start: u64(p)?,
                end: None,
                address_type: AddressType::I32,
            },
            0x01 => Self {
                start: u64(p)?,
                end: Some(u64(p)?),
                address_type: AddressType::I32,
            },
            0x04 => Self {
                start: u64(p)?,
                end: None,
                address_type: AddressType::I64,
            },
            0x05 => Self {
                start: u64(p)?,
                end: Some(u64(p)?),
                address_type: AddressType::I64,
            },
            _ => return Err(Error),
        })
    }
}

impl TagType {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        let 0x00 = u8(p)? else {
            return Err(Error);
        };
        TypeIdx::parse(p).map(Self)
    }
}

impl GlobalType {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        Ok(Self {
            value_type: ValType::parse(p)?,
            mutability: Mut::parse(p).ok(),
        })
    }
}

impl MemType {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        Limits::parse(p).map(Self)
    }
}

impl TableType {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        Ok(Self {
            ref_type: RefType::parse(p)?,
            limits: Limits::parse(p)?,
        })
    }
}

impl ExternType {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        match u8(p)? {
            0x00 => TypeIdx::parse(p).map(Self::Func),
            0x01 => TableType::parse(p).map(Self::Table),
            0x02 => MemType::parse(p).map(Self::Mem),
            0x03 => GlobalType::parse(p).map(Self::Global),
            0x04 => TagType::parse(p).map(Self::Tag),
            _ => Err(Error),
        }
    }
}

fn byte_array<const N: usize>(p: &mut Parser) -> Result<[u8; N], Error> {
    let slice = p.input.split_off(..N).ok_or(Error)?;
    Ok(slice.try_into().unwrap())
}

fn u8(p: &mut Parser) -> Result<u8, Error> {
    p.input.split_off_first().copied().ok_or(Error)
}

fn uleb128<T>(p: &mut Parser) -> Result<T, Error>
where
    T: Default + From<u8> + core::ops::BitOrAssign + core::ops::Shl<usize, Output = T>,
{
    let mut n = T::default();
    for shift in (0..const { size_of::<T>() * 8 }).step_by(7) {
        let byte = u8(p)?;
        #[expect(
            clippy::arithmetic_side_effects,
            reason = "Shift amount is bounded by size of `T`"
        )]
        {
            n |= T::from(byte & !(1 << 7)) << shift;
        }
        if byte & (1 << 7) == 0 {
            return Ok(n);
        }
    }
    Err(Error)
}

fn leb128_s33_positive(p: &mut Parser) -> Result<u32, Error> {
    let mut n = 0;
    for shift in (0..33_usize).step_by(7) {
        let byte = u8(p)?;
        n |= i64::from(byte & !(1 << 7)) << shift;
        if byte & (1 << 7) == 0 {
            if byte & (1 << 6) != 0 {
                return Err(Error);
            }
            return n.try_into().map_err(|_| Error);
        }
    }
    Err(Error)
}

fn u32(p: &mut Parser) -> Result<u32, Error> {
    uleb128(p)
}

fn u64(p: &mut Parser) -> Result<u64, Error> {
    uleb128(p)
}

impl Name {
    fn parse(p: &mut Parser) -> Result<Self, Error> {
        let bytes: Vec<u8> = vec(u8, p)?;
        bytes.try_into().map_err(|_| Error).map(Name)
    }
}

fn byte(b: u8, p: &mut Parser) -> bool {
    p.input
        .strip_prefix(&[b])
        .inspect(|it| p.input = it)
        .is_some()
}

fn vec<T>(
    element: impl Fn(&mut Parser) -> Result<T, Error>,
    p: &mut Parser,
) -> Result<Vec<T>, Error> {
    let len = u32(p)?;
    let iter = core::iter::repeat_with(|| element(p));
    iter.take(usize(len)).collect()
}

const fn usize(n: u32) -> usize {
    const { assert!(size_of::<u32>() <= size_of::<usize>()) }
    n as usize
}
