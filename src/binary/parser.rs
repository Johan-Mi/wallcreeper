use super::{instructions, modules, types, values};
use alloc::vec::Vec;

pub struct Error;

#[expect(clippy::too_many_lines, reason = "Bytecode parsing")]
impl instructions::Instr {
    fn parse(input: &mut &[u8]) -> Result<Self, Error> {
        use instructions::{BlockType, MemArg};
        use modules::{
            DataIdx, ElemIdx, FuncIdx, GlobalIdx, LabelIdx, LocalIdx, MemIdx, TableIdx, TagIdx,
            TypeIdx,
        };
        use types::{HeapType, ValType};

        fn flip<A, B, R>(f: impl Fn(A, B) -> R) -> impl Fn(B, A) -> R {
            move |b, a| f(a, b)
        }

        Ok(match u8(input)? {
            0x00 => Self::Unreachable,
            0x01 => Self::Nop,
            0x02 => {
                let r#type = BlockType::parse(input)?;
                let mut instrs = Vec::new();
                while !byte(0x0b, input) {
                    if input.is_empty() {
                        return Err(Error);
                    }
                    instrs.push(Self::parse(input)?);
                }
                Self::Block(r#type, instrs)
            }
            0x03 => {
                let r#type = BlockType::parse(input)?;
                let mut instrs = Vec::new();
                while !byte(0x0b, input) {
                    if input.is_empty() {
                        return Err(Error);
                    }
                    instrs.push(Self::parse(input)?);
                }
                Self::Loop(r#type, instrs)
            }
            0x04 => todo!("if bt"),
            0x05 => return Err(Error), // else
            0x06..=0x07 => return Err(Error),
            0x08 => Self::Throw(TagIdx::parse(input)?),
            0x09 => return Err(Error),
            0x0a => Self::ThrowRef,
            0x0b => return Err(Error), // end
            0x0c => Self::Br(LabelIdx::parse(input)?),
            0x0d => Self::BrIf(LabelIdx::parse(input)?),
            0x0e => Self::BrTable(vec(LabelIdx::parse, input)?, LabelIdx::parse(input)?),
            0x0f => Self::Return,
            0x10 => Self::Call(FuncIdx::parse(input)?),
            0x11 => flip(Self::CallIndirect)(TypeIdx::parse(input)?, TableIdx::parse(input)?),
            0x12 => Self::ReturnCall(FuncIdx::parse(input)?),
            0x13 => flip(Self::ReturnCallIndirect)(TypeIdx::parse(input)?, TableIdx::parse(input)?),
            0x14 => Self::CallRef(TypeIdx::parse(input)?),
            0x15 => Self::ReturnCallRef(TypeIdx::parse(input)?),
            0x16..=0x19 => return Err(Error),
            0x1a => Self::Drop,
            0x1b => Self::Select(Vec::new()),
            0x1c => Self::Select(vec(ValType::parse, input)?),
            0x1d..=0x1e => return Err(Error),
            0x1f => todo!("try_table bt"),
            0x20 => Self::Local·Get(LocalIdx::parse(input)?),
            0x21 => Self::Local·Set(LocalIdx::parse(input)?),
            0x22 => Self::Local·Tee(LocalIdx::parse(input)?),
            0x23 => Self::Global·Get(GlobalIdx::parse(input)?),
            0x24 => Self::Global·Set(GlobalIdx::parse(input)?),
            0x25 => Self::Table·Get(TableIdx::parse(input)?),
            0x26 => Self::Table·Set(TableIdx::parse(input)?),
            0x27 => return Err(Error),
            0x28 => Self::I32·Load(MemArg::parse(input)?),
            0x29 => Self::I64·Load(MemArg::parse(input)?),
            0x2a => Self::F32·Load(MemArg::parse(input)?),
            0x2b => Self::F64·Load(MemArg::parse(input)?),
            0x2c => Self::I32·Load8S(MemArg::parse(input)?),
            0x2d => Self::I32·Load8U(MemArg::parse(input)?),
            0x2e => Self::I32·Load16S(MemArg::parse(input)?),
            0x2f => Self::I32·Load16U(MemArg::parse(input)?),
            0x30 => Self::I64·Load8S(MemArg::parse(input)?),
            0x31 => Self::I64·Load8U(MemArg::parse(input)?),
            0x32 => Self::I64·Load16S(MemArg::parse(input)?),
            0x33 => Self::I64·Load16U(MemArg::parse(input)?),
            0x34 => Self::I64·Load32S(MemArg::parse(input)?),
            0x35 => Self::I64·Load32U(MemArg::parse(input)?),
            0x36 => Self::I32·Store(MemArg::parse(input)?),
            0x37 => Self::I64·Store(MemArg::parse(input)?),
            0x38 => Self::F32·Store(MemArg::parse(input)?),
            0x39 => Self::F64·Store(MemArg::parse(input)?),
            0x3a => Self::I32·Store8(MemArg::parse(input)?),
            0x3b => Self::I32·Store16(MemArg::parse(input)?),
            0x3c => Self::I64·Store8(MemArg::parse(input)?),
            0x3d => Self::I64·Store16(MemArg::parse(input)?),
            0x3e => Self::I64·Store32(MemArg::parse(input)?),
            0x3f => Self::Memory·Size(MemIdx::parse(input)?),
            0x40 => Self::Memory·Grow(MemIdx::parse(input)?),
            0x41 => Self::I32·Const(i32::from_le_bytes(byte_array(input)?)),
            0x42 => Self::I64·Const(i64::from_le_bytes(byte_array(input)?)),
            0x43 => Self::F32·Const(f32::from_le_bytes(byte_array(input)?)),
            0x44 => Self::F64·Const(f64::from_le_bytes(byte_array(input)?)),
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
            0xc5..=0xcf => return Err(Error),
            0xd0 => Self::Ref·Null(HeapType::parse(input)?),
            0xd1 => Self::Ref·IsNull,
            0xd2 => Self::Ref·Func(FuncIdx::parse(input)?),
            0xd3 => Self::Ref·Eq,
            0xd4 => Self::Ref·AsNonNull,
            0xd5 => Self::BrOnNull(LabelIdx::parse(input)?),
            0xd6 => Self::BrOnNonNull(LabelIdx::parse(input)?),
            0xd7..=0xfa => return Err(Error),
            0xfb => Self::aggregate(input)?,
            0xfc => match u32(input)? {
                0 => Self::I32·TruncSatSF32,
                1 => Self::I32·TruncSatUF32,
                2 => Self::I32·TruncSatSF64,
                3 => Self::I32·TruncSatUF64,
                4 => Self::I64·TruncSatSF32,
                5 => Self::I64·TruncSatUF32,
                6 => Self::I64·TruncSatSF64,
                7 => Self::I64·TruncSatUF64,
                8 => flip(Self::Memory·Init)(DataIdx::parse(input)?, MemIdx::parse(input)?),
                9 => Self::Data·Drop(DataIdx::parse(input)?),
                10 => Self::Memory·Copy(MemIdx::parse(input)?, MemIdx::parse(input)?),
                11 => Self::Memory·Fill(MemIdx::parse(input)?),
                12 => flip(Self::Table·Init)(ElemIdx::parse(input)?, TableIdx::parse(input)?),
                13 => Self::Elem·Drop(ElemIdx::parse(input)?),
                14 => Self::Table·Copy(TableIdx::parse(input)?, TableIdx::parse(input)?),
                15 => Self::Table·Grow(TableIdx::parse(input)?),
                16 => Self::Table·Size(TableIdx::parse(input)?),
                17 => Self::Table·Fill(TableIdx::parse(input)?),
                _ => return Err(Error),
            },
            0xfd => Self::vector(input)?,
            0xfe..=0xff => return Err(Error),
        })
    }

    fn aggregate(input: &mut &[u8]) -> Result<Self, Error> {
        use instructions::Nullable;
        use modules::{DataIdx, ElemIdx, FieldIdx, TypeIdx};
        use types::{HeapType, RefType};

        Ok(match u32(input)? {
            0 => Self::Struct·New(TypeIdx::parse(input)?),
            1 => Self::Struct·NewDefault(TypeIdx::parse(input)?),
            2 => Self::Struct·Get(TypeIdx::parse(input)?, FieldIdx::parse(input)?),
            3 => Self::Struct·GetS(TypeIdx::parse(input)?, FieldIdx::parse(input)?),
            4 => Self::Struct·GetU(TypeIdx::parse(input)?, FieldIdx::parse(input)?),
            5 => Self::Struct·Set(TypeIdx::parse(input)?, FieldIdx::parse(input)?),
            6 => Self::Array·New(TypeIdx::parse(input)?),
            7 => Self::Array·NewDefault(TypeIdx::parse(input)?),
            8 => Self::Array·NewFixed(TypeIdx::parse(input)?, u32(input)?),
            9 => Self::Array·NewData(TypeIdx::parse(input)?, DataIdx::parse(input)?),
            10 => Self::Array·NewElem(TypeIdx::parse(input)?, ElemIdx::parse(input)?),
            11 => Self::Array·Get(TypeIdx::parse(input)?),
            12 => Self::Array·GetS(TypeIdx::parse(input)?),
            13 => Self::Array·GetU(TypeIdx::parse(input)?),
            14 => Self::Array·Set(TypeIdx::parse(input)?),
            15 => Self::Array·Len,
            16 => Self::Array·Fill(TypeIdx::parse(input)?),
            17 => Self::Array·Copy(TypeIdx::parse(input)?, TypeIdx::parse(input)?),
            18 => Self::Array·InitData(TypeIdx::parse(input)?, DataIdx::parse(input)?),
            19 => Self::Array·InitElem(TypeIdx::parse(input)?, ElemIdx::parse(input)?),
            20 => Self::Ref·Test(RefType {
                r#type: HeapType::parse(input)?,
                nullability: Nullable(false),
            }),
            21 => Self::Ref·Test(RefType {
                r#type: HeapType::parse(input)?,
                nullability: Nullable(true),
            }),
            22 => Self::Ref·Cast(RefType {
                r#type: HeapType::parse(input)?,
                nullability: Nullable(false),
            }),
            23 => Self::Ref·Cast(RefType {
                r#type: HeapType::parse(input)?,
                nullability: Nullable(true),
            }),
            26 => Self::Any·ConvertExtern,
            27 => Self::Extern·ConvertAny,
            28 => Self::Ref·I31,
            29 => Self::I31·GetS,
            30 => Self::I31·GetU,
            _ => return Err(Error),
        })
    }

    fn vector(input: &mut &[u8]) -> Result<Self, Error> {
        use instructions::{LaneIdx, MemArg};

        Ok(match u32(input)? {
            0 => Self::V128·Load(MemArg::parse(input)?),
            1 => Self::V128·Load8x8S(MemArg::parse(input)?),
            2 => Self::V128·Load8x8U(MemArg::parse(input)?),
            3 => Self::V128·Load16x4S(MemArg::parse(input)?),
            4 => Self::V128·Load16x4U(MemArg::parse(input)?),
            5 => Self::V128·Load32x2S(MemArg::parse(input)?),
            6 => Self::V128·Load32x2U(MemArg::parse(input)?),
            7 => Self::V128·Load8Splat(MemArg::parse(input)?),
            8 => Self::V128·Load16Splat(MemArg::parse(input)?),
            9 => Self::V128·Load32Splat(MemArg::parse(input)?),
            10 => Self::V128·Load64Splat(MemArg::parse(input)?),
            11 => Self::V128·Store(MemArg::parse(input)?),
            12 => Self::V128·Const(u128::from_le_bytes(byte_array(input)?)),
            13 => Self::I8x16·Shuffle(byte_array(input)?.map(LaneIdx)),
            14 => Self::I8x16·Swizzle,
            15 => Self::I8x16·Splat,
            16 => Self::I16x8·Splat,
            17 => Self::I32x4·Splat,
            18 => Self::I64x2·Splat,
            19 => Self::F32x4·Splat,
            20 => Self::F64x2·Splat,
            21 => Self::I8x16·ExtractLaneS(LaneIdx::parse(input)?),
            22 => Self::I8x16·ExtractLaneU(LaneIdx::parse(input)?),
            23 => Self::I8x16·ReplaceLane(LaneIdx::parse(input)?),
            24 => Self::I16x8·ExtractLaneS(LaneIdx::parse(input)?),
            25 => Self::I16x8·ExtractLaneU(LaneIdx::parse(input)?),
            26 => Self::I16x8·ReplaceLane(LaneIdx::parse(input)?),
            27 => Self::I32x4·ExtractLane(LaneIdx::parse(input)?),
            28 => Self::I32x4·ReplaceLane(LaneIdx::parse(input)?),
            29 => Self::I64x2·ExtractLane(LaneIdx::parse(input)?),
            30 => Self::I64x2·ReplaceLane(LaneIdx::parse(input)?),
            31 => Self::F32x4·ExtractLane(LaneIdx::parse(input)?),
            32 => Self::F32x4·ReplaceLane(LaneIdx::parse(input)?),
            33 => Self::F64x2·ExtractLane(LaneIdx::parse(input)?),
            34 => Self::F64x2·ReplaceLane(LaneIdx::parse(input)?),
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
            84 => Self::V128·Load8Lane(MemArg::parse(input)?, LaneIdx::parse(input)?),
            85 => Self::V128·Load16Lane(MemArg::parse(input)?, LaneIdx::parse(input)?),
            86 => Self::V128·Load32Lane(MemArg::parse(input)?, LaneIdx::parse(input)?),
            87 => Self::V128·Load64Lane(MemArg::parse(input)?, LaneIdx::parse(input)?),
            88 => Self::V128·Store8Lane(MemArg::parse(input)?, LaneIdx::parse(input)?),
            89 => Self::V128·Store16Lane(MemArg::parse(input)?, LaneIdx::parse(input)?),
            90 => Self::V128·Store32Lane(MemArg::parse(input)?, LaneIdx::parse(input)?),
            91 => Self::V128·Store64Lane(MemArg::parse(input)?, LaneIdx::parse(input)?),
            92 => Self::V128·Load32Zero(MemArg::parse(input)?),
            93 => Self::V128·Load64Zero(MemArg::parse(input)?),
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
        let true = align < 1 << 7 else {
            return Err(Error);
        };
        let (memory, align) = if let Some(align) = align.checked_sub(1 << 6) {
            (MemIdx::parse(input)?, align)
        } else {
            (MemIdx(0), align)
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
        while !byte(0x0b, input) {
            if input.is_empty() {
                return Err(Error);
            }
            instrs.push(instructions::Instr::parse(input)?);
        }
        Ok(Self(instrs))
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
            let r#type = types::TableType::parse(input)?;
            let initializer =
                instructions::Expr([instructions::Instr::Ref·Null(r#type.ref_type.r#type)].into());
            Ok(Self {
                r#type,
                initializer,
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
    pub fn parse(input: &[u8]) -> Result<Self, Error> {
        let mut input = input.strip_prefix(b"\0asm\x01\0\0\0").ok_or(Error)?;

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

            let section_id = SectionId::parse(&mut input)?;
            if section_id != SectionId::Custom {
                if Some(section_id) <= section_id_prev {
                    return Err(Error);
                }
                section_id_prev = Some(section_id);
            }

            let len_wanted = usize(u32(&mut input)?);
            let len_before = input.len();
            match section_id {
                SectionId::Custom => {
                    customsecs.push(modules::Custom::parse(len_wanted, &mut input)?);
                }
                SectionId::Type => typesec = vec(types::RecType::parse, &mut input)?,
                SectionId::Import => importsec = vec(modules::Import::parse, &mut input)?,
                SectionId::Function => funcsec = vec(modules::TypeIdx::parse, &mut input)?,
                SectionId::Table => tablesec = vec(modules::Table::parse, &mut input)?,
                SectionId::Memory => memsec = vec(types::MemType::parse, &mut input)?,
                SectionId::Tag => tagsec = vec(types::TagType::parse, &mut input)?,
                SectionId::Global => globalsec = vec(modules::Global::parse, &mut input)?,
                SectionId::Export => exportsec = vec(modules::Export::parse, &mut input)?,
                SectionId::Start => startsec = Some(modules::FuncIdx::parse(&mut input)?),
                SectionId::Element => elemsec = vec(modules::Elem::parse, &mut input)?,
                SectionId::DataCount => datacntsec = Some(u32(&mut input)?),
                SectionId::Code => codesec = vec(modules::Code::parse, &mut input)?,
                SectionId::Data => datasec = vec(modules::Data::parse, &mut input)?,
            }
            if len_wanted != len_before.strict_sub(input.len()) {
                return Err(Error);
            }
        }

        if !input.is_empty() {
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
    for shift in (0..const { size_of::<T>() * 8 }).step_by(7) {
        let byte = u8(input)?;
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

fn leb128_s33_positive(input: &mut &[u8]) -> Result<u32, Error> {
    let mut n = 0;
    for shift in (0..33_usize).step_by(7) {
        let byte = u8(input)?;
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
