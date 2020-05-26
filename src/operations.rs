#[derive(Debug, Eq, PartialEq)]
pub enum UndefinedOperation {
    OpType,
    Kind,
    Variant,
    Mode,
    ParameterMode,
}

pub mod op_codes {
    pub const NOP: u8 = 0x00;
    pub const END: u8 = 0x01;
    pub const SLP: u8 = 0x02;
    pub const SET: u8 = 0x03;
    pub const ADD: u8 = 0x04;
    pub const SUB: u8 = 0x05;
    pub const MUL: u8 = 0x06;
    pub const DIV: u8 = 0x07;
    pub const MOD: u8 = 0x08;
    pub const SHL: u8 = 0x09;
    pub const SHR: u8 = 0x0A;
    pub const AND: u8 = 0x0B;
    pub const OR: u8 = 0x0C;
    pub const XOR: u8 = 0x0D;
    pub const NOT: u8 = 0x0E;
    pub const NEG: u8 = 0x0F;
    pub const INC: u8 = 0x10;
    pub const DEC: u8 = 0x11;

    pub const PSF: u8 = 0x20;
    pub const PAR: u8 = 0x21;
    pub const CFN: u8 = 0x22;
}

#[derive(Debug, Eq, PartialEq)]
pub enum Operand {
    /// Local variable.
    ///
    /// Expressed as `x` or `loc(12)`.
    Loc(usize),

    /// Indirection access.
    ///
    /// Expressed as `*x` or `ind(12)`.
    Ind(usize),

    /// Return variable.
    ///
    /// Expressed as `^x` or `ret(12)`.
    Ret(usize),

    /// Constant value.
    ///
    /// Expressed as `12` or `val(12)`.
    Val(usize),

    /// Variable reference.
    ///
    /// Expressed as `&x` or `ref(12)`.
    Ref(usize),

    /// Empty.
    ///
    /// Expressed as `emp`.
    Emp,
}

impl Operand {
    pub fn new(value: usize, kind: u8) -> Result<Self, UndefinedOperation> {
        use Operand::*;

        Ok(match kind {
            0 => Loc(value),
            1 => Ind(value),
            2 => Ret(value),
            3 => Val(value),
            4 => Ref(value),
            5 => Emp,
            _ => return Err(UndefinedOperation::Kind),
        })
    }
}

impl From<u8> for Operand {
    fn from(byte: u8) -> Self { Operand::Loc(byte as usize) }
}

#[derive(Debug, Eq, PartialEq)]
pub struct UnOp {
    x: Operand,
    x_offset: Option<Operand>,
}

impl UnOp {
    pub fn new(x: Operand) -> Self { Self { x, x_offset: None } }

    pub fn with_x_offset(mut self, x_offset: Operand) -> Self {
        self.x_offset = Some(x_offset);
        self
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct BinOp {
    x: Operand,
    x_offset: Option<Operand>,
    y: Operand,
    y_offset: Option<Operand>,
}

impl BinOp {
    pub fn new(x: Operand, y: Operand) -> Self {
        Self {
            x,
            x_offset: None,
            y,
            y_offset: None,
        }
    }

    pub fn with_x_offset(mut self, x_offset: Operand) -> Self {
        self.x_offset = Some(x_offset);
        self
    }

    pub fn with_y_offset(mut self, y_offset: Operand) -> Self {
        self.y_offset = Some(y_offset);
        self
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Op {
    Nop,
    End(UnOp),
    Slp(UnOp),
    Set(BinOp, OpType),
    Add(BinOp, OpType, Mode),
    Sub(BinOp, OpType, Mode),
    Mul(BinOp, OpType, Mode),
    Div(BinOp, OpType),
    Mod(BinOp, OpType),
    Shl(BinOp, OpType, Mode),
    Shr(BinOp, OpType, Mode),
    And(BinOp, OpType),
    Or(BinOp, OpType),
    Xor(BinOp, OpType),
    Not(UnOp, OpType),
    Neg(UnOp, OpType, Mode),
    Inc(UnOp, OpType, Mode),
    Dec(UnOp, OpType, Mode),

    Psf(UnOp),
    Par(UnOp, OpType, ParameterMode),
    Cfn(UnOp),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Spec {
    pub op_type: OpType,
    pub mode: Mode,
    pub variant: Variant,
}

#[derive(Debug, Eq, PartialEq)]
pub enum OpType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    Uw,
    Iw,
    F32,
    F64,
}

impl OpType {
    pub fn new(value: u8) -> Result<Self, UndefinedOperation> {
        use OpType::*;

        Ok(match value {
            0 => U8,
            1 => I8,
            2 => U16,
            3 => I16,
            4 => U32,
            5 => I32,
            6 => U64,
            7 => I64,
            8 => Uw,
            9 => Iw,
            11 => F32,
            13 => F64,
            _ => return Err(UndefinedOperation::OpType),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Mode {
    /// Wrapping mode.
    Wrap,

    /// Saturating mode.
    Sat,

    /// Wide mode.
    Wide,

    /// Handling mode.
    Hand,
}

impl Mode {
    pub fn new(value: u8) -> Result<Self, UndefinedOperation> {
        use Mode::*;

        Ok(match value {
            0 => Wrap,
            1 => Sat,
            2 => Wide,
            3 => Hand,
            _ => return Err(UndefinedOperation::Mode),
        })
    }
}

impl Default for Mode {
    fn default() -> Self { Mode::Wrap }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Variant {
    /// `x y` variant.
    XY,

    /// `x:q y` variant.
    XOffsetY,

    /// `x y:q` variant.
    XYOffset,

    /// `x:q y:w` variant.
    XOffsetYOffset,
}

impl Variant {
    pub fn new(variant: u8) -> Result<Self, UndefinedOperation> {
        use Variant::*;

        Ok(match variant {
            0 => XY,
            1 => XOffsetY,
            2 => XYOffset,
            3 => XOffsetYOffset,
            _ => return Err(UndefinedOperation::Variant),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParameterMode {
    /// Set mode.
    Set,

    /// Empty mode.
    Emp,

    /// Memory set zeroes mode.
    Msz,
}

impl ParameterMode {
    pub fn new(value: u8) -> Result<Self, UndefinedOperation> {
        use ParameterMode::*;

        Ok(match value {
            0 => Set,
            1 => Emp,
            2 => Msz,
            _ => return Err(UndefinedOperation::ParameterMode),
        })
    }
}
