use std::collections::HashMap;

mod elf;
mod instruct_table;

#[derive(Debug,PartialEq,Clone)]
pub struct Context {
    name_space: Vec<String>,
    functions_map: HashMap<String,(FunctionDecArgs,AstmType)>
}
impl Context {
    pub fn new() -> Self {
        Self {
            name_space: Vec::new(),
            functions_map: HashMap::new(),
        }
    }
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub enum AstmType {
    U8,I8,U16,I16,U32,I32,U64,I64,Ptr,Void
}

#[derive(Debug, Clone, Default)]
pub struct AstmBlock {}

pub struct AstmFunction {
    return_type: AstmType,
    arguments: FunctionDecArgs,
    block: AstmBlock,
}
impl AstmFunction {
    pub fn declare(context: &mut Context,name: impl ToString, args: FunctionDecArgs, return_type: AstmType) -> Self {
        context.name_space.push(name.to_string().clone());
        context.functions_map.insert(name.to_string().clone(),(args.clone(),return_type.clone()));
        Self {
            arguments: args,
            return_type,
            block: AstmBlock::default(),
        }
    }
}

#[derive(Debug,PartialEq,Clone,Default)]
pub struct FunctionDecArgs {}
pub struct Arg {}


fn main() {
    let mut ctx = Context::new();
    let main_function = AstmFunction::declare(&mut ctx, "_start", FunctionDecArgs::default(), AstmType::Void);
}
