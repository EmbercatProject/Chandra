use std::collections::HashMap;

use crate::core::{types::{Value, Computable, Void}, operation::{Operation, OperationWrapper}};

pub mod calc;
pub mod compare;
pub mod control_flow;
pub mod structure;
pub mod variables;

pub trait GPUOperation<R: GPUValue>: Operation<R> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String;
}

impl<R: GPUValue, O: GPUOperation<R>> GPUOperation<R> for OperationWrapper<R, O> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        self.0.build(functions)
    }
}

pub trait GPUValue: Value {
    fn get_return_decl() -> String;
}

pub trait GPUComputable: GPUValue + Computable {
    fn get_type_info() -> String;
}

impl GPUValue for Void {
    fn get_return_decl() -> String {
        String::new()
    }
}

impl GPUValue for bool {
    fn get_return_decl() -> String {
        "-> bool".to_string()
    }
}
impl GPUComputable for bool {
    fn get_type_info() -> String {
        "bool".to_string()
    }
}

impl GPUValue for i32 {
    fn get_return_decl() -> String {
       "-> i32".to_string()
    }
}
impl GPUComputable for i32 {
    fn get_type_info() -> String {
        "i32".to_string()
    }
}

impl GPUValue for u32 {
    fn get_return_decl() -> String {
       "-> u32".to_string()
    }
}
impl GPUComputable for u32 {
    fn get_type_info() -> String {
        "u32".to_string()
    }
}

impl GPUValue for f32 {
    fn get_return_decl() -> String {
       "-> f32".to_string()
    }
}
impl GPUComputable for f32 {
    fn get_type_info() -> String {
        "f32".to_string()
    }
}