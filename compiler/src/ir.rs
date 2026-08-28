//! Backend-neutral Universal IR skeleton.
//!
//! V0.1 executes the checked AST, but this module fixes the architectural
//! boundary for later typed lowering without tying the language to LLVM,
//! JavaScript, WASM, SQL, or any other target.

#[derive(Debug, Clone, PartialEq)]
pub struct IrProgram {
    pub functions: Vec<IrFunction>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrFunction {
    pub name: String,
    pub blocks: Vec<BasicBlock>,
    pub entry: BlockId,
}

pub type BlockId = u32;
pub type ValueId = u32;

#[derive(Debug, Clone, PartialEq)]
pub struct BasicBlock {
    pub id: BlockId,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Const { out: ValueId, value: Constant },
    LoadName { out: ValueId, name: String },
    StoreName { name: String, value: ValueId },
    LoadProperty { out: ValueId, object: ValueId, property: String },
    HasPropertyValue { out: ValueId, object: ValueId, property: String },
    Call { out: Option<ValueId>, callee: String, args: Vec<ValueId> },
    EmitValidation { message: ValueId },
    EmitAction { words: Vec<String> },
    Print { value: ValueId },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Terminator {
    Jump(BlockId),
    Branch { condition: ValueId, when_true: BlockId, when_false: BlockId },
    Return(Option<ValueId>),
    Stop,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Text(String),
    Integer(i64),
    Decimal(String),
    Boolean(bool),
    Null,
}
