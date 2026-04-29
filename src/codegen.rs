use crate::parser::{
    ast::{Expression, Statement},
    Parser,
};

#[derive(Debug)]
pub struct Codegen {
    root_node: Box<Statement>,
}

impl Codegen {
    fn new(root_node: Box<Statement>) -> Self {
        Self { root_node }
    }

    fn start(&mut self) {}
    fn function(&mut self) {}
    fn end(&mut self) {}
}

impl From<Box<Statement>> for Codegen {
    fn from(value: Box<Statement>) -> Self {
        Self::new(value)
    }
}

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("quetzal-lang/src-cpp/llvm_bridge.hpp");
        fn quetzal_init_llvm();
    }
}
