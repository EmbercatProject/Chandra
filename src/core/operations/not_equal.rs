use std::marker::PhantomData;

use crate::core::{
    operation::{Operation, OperationWrapper},
    types::Computable, processor::cpu::DifferentiatedCPUContext, 
};

pub fn not_qual<R: Computable + std::cmp::PartialEq, LEFT: Operation<R>, RIGHT: Operation<R>>(
    left: LEFT,
    right: RIGHT,
) -> OperationWrapper<bool, NotEqual<R, LEFT, RIGHT>> {
    OperationWrapper(
        NotEqual {
            left: left,
            right: right,
            _0: PhantomData,
        },
        PhantomData,
    )
}

#[derive(Clone, Debug)]
pub struct NotEqual<R: Computable + std::cmp::PartialEq, LEFT: Operation<R>, RIGHT: Operation<R>> {
    pub left: LEFT,
    pub right: RIGHT,
    _0: PhantomData<R>,
}

impl<R: Computable + std::cmp::PartialEq, LEFT: Operation<R>, RIGHT: Operation<R>> Operation<bool> for NotEqual<R, LEFT, RIGHT> {
    fn evaluate(&self, context: &mut DifferentiatedCPUContext) -> bool {
        self.left.evaluate(context) != self.right.evaluate(context)
    }
}
