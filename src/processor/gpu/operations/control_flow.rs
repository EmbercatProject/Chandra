use std::collections::HashMap;

use crate::core::{types::{Void}, operation::OperationWrapper, operations::{range::Range, returns::Returns, if_else::IfElse, foreach::ForEach}, type_traits::Iterable};

use super::{GPUOperation, GPUValue};


impl<R: GPUValue, O: GPUOperation<R>> GPUOperation<R> for Returns<R, O> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        format!("return {};", self.operation.build(functions))
    }
}

impl<CONDITION: GPUOperation<bool>, A: GPUOperation<Void>, B: GPUOperation<Void>, C: GPUOperation<Void>, D: GPUOperation<Void>> GPUOperation<Void> for IfElse<Void, CONDITION, A, B, C, D> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let condition = self.condition.build(functions);
        let then = self.then.build(functions);
        if let Some(els) = &self.els {
            let el = els.build(functions);
            format!("if ({}) {} else {} \n", condition, then, el)
        } else {
            format!("if ({}) {} \n", condition, then)
        }
    }
}

impl<A: GPUOperation<u32>, B: GPUOperation<u32>> GPUOperation<u32> for Range<A, B> {
    fn build(&self, _functions: &mut HashMap<String, String>) -> String {
        unimplemented!()
    }
}

impl<LEFT: GPUOperation<u32>, RIGHT: GPUOperation<u32>, A: GPUOperation<Void>, B: GPUOperation<Void>> GPUOperation<Void> for ForEach<u32, OperationWrapper<u32, Range<LEFT, RIGHT>>, A, B> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let start = self.iterable.0.get_start().build(functions);
        let end = self.iterable.0.get_boundry().build(functions);
        let block = self.scope.build(functions);
        //for (var i: i32 = 0; i < 4; i++)
        format!("for (var loop_var: u32 = {}; loop_var < {}; loop_var++) {}", start, end, block)
    }
}