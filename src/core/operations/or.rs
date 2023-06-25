use std::marker::PhantomData;

use crate::core::{
    operation::{Operation, OperationWrapper},
    processor::cpu::DifferentiatedCPUContext, 
};

pub fn or<LEFT: Operation<bool>, RIGHT: Operation<bool>>(
    left: LEFT,
    right: RIGHT,
) -> OperationWrapper<bool, Or<LEFT, RIGHT>> {
    OperationWrapper(
        Or {
            left: left,
            right: right,
        },
        PhantomData,
    )
}

#[derive(Clone, Debug)]
pub struct Or<LEFT: Operation<bool>, RIGHT: Operation<bool>> {
    pub left: LEFT,
    pub right: RIGHT,
}

impl<LEFT: Operation<bool>, RIGHT: Operation<bool>> Operation<bool> for Or<LEFT, RIGHT> {
    fn evaluate(&self, context: &mut DifferentiatedCPUContext) -> bool {
        self.left.evaluate(context) || self.right.evaluate(context)
     }
}
