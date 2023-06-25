use std::marker::PhantomData;

use crate::core::{
    operation::{Operation, OperationWrapper},
    types::Computable, processor::cpu::DifferentiatedCPUContext, 
};

pub fn greater_than<R: Computable + std::cmp::PartialOrd, LEFT: Operation<R>, RIGHT: Operation<R>>(
    left: LEFT,
    right: RIGHT,
) -> OperationWrapper<bool, GreaterThan<R, LEFT, RIGHT>> {
    OperationWrapper(
        GreaterThan {
            left: left,
            right: right,
            _0: PhantomData,
        },
        PhantomData,
    )
}

#[derive(Clone, Debug)]
pub struct GreaterThan<R: Computable + std::cmp::PartialOrd, LEFT: Operation<R>, RIGHT: Operation<R>> {
    pub left: LEFT,
    pub right: RIGHT,
    _0: PhantomData<R>,
}

impl<R: Computable + std::cmp::PartialOrd, LEFT: Operation<R>, RIGHT: Operation<R>> Operation<bool> for GreaterThan<R, LEFT, RIGHT> {
    fn evaluate(&self, context: &mut DifferentiatedCPUContext) -> bool {
        self.left.evaluate(context) > self.right.evaluate(context)
     }
}
