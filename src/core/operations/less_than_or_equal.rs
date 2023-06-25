use std::marker::PhantomData;

use crate::core::{
    operation::{Operation, OperationWrapper},
    types::Computable, processor::cpu::DifferentiatedCPUContext
};

pub fn less_than_or_equal<R: Computable + std::cmp::PartialOrd, LEFT: Operation<R>, RIGHT: Operation<R>>(
    left: LEFT,
    right: RIGHT,
) -> OperationWrapper<bool, LessThanOrEqual<R, LEFT, RIGHT>> {
    OperationWrapper(
        LessThanOrEqual {
            left: left,
            right: right,
            _0: PhantomData,
        },
        PhantomData,
    )
}

#[derive(Clone, Debug)]
pub struct LessThanOrEqual<R: Computable + std::cmp::PartialOrd, LEFT: Operation<R>, RIGHT: Operation<R>> {
    pub left: LEFT,
    pub right: RIGHT,
    _0: PhantomData<R>,
}

impl<R: Computable + std::cmp::PartialOrd, LEFT: Operation<R>, RIGHT: Operation<R>> Operation<bool> for LessThanOrEqual<R, LEFT, RIGHT> {
    fn evaluate(&self, context: &mut DifferentiatedCPUContext) -> bool {
        self.left.evaluate(context) <= self.right.evaluate(context)
     }
}
