use std::marker::PhantomData;

use crate::core::{
    operation::{Operation, OperationWrapper},
    types::Computable, processor::cpu::DifferentiatedCPUContext, 
};

pub fn less_than<R: Computable + std::cmp::PartialOrd, LEFT: Operation<R>, RIGHT: Operation<R>>(
    left: LEFT,
    right: RIGHT,
) -> OperationWrapper<bool, LessThan<R, LEFT, RIGHT>> {
    OperationWrapper(
        LessThan {
            left: left,
            right: right,
            _0: PhantomData,
        },
        PhantomData,
    )
}

#[derive(Clone, Debug)]
pub struct LessThan<R: Computable + std::cmp::PartialOrd, LEFT: Operation<R>, RIGHT: Operation<R>> {
    pub left: LEFT,
    pub right: RIGHT,
    _0: PhantomData<R>,
}

impl<R: Computable + std::cmp::PartialOrd, LEFT: Operation<R>, RIGHT: Operation<R>> Operation<bool> for LessThan<R, LEFT, RIGHT> {
    fn evaluate(&self, context: &mut DifferentiatedCPUContext) -> bool {
        self.left.evaluate(context) < self.right.evaluate(context)
    }
}
