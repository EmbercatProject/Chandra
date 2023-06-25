use std::marker::PhantomData;

use crate::core::{
    operation::{Operation, OperationWrapper}, processor::cpu::DifferentiatedCPUContext
};


pub fn and<LEFT: Operation<bool>, RIGHT: Operation<bool>>(
    left: LEFT,
    right: RIGHT,
) -> OperationWrapper<bool, And<LEFT, RIGHT>> {
    OperationWrapper(
        And {
            left: left,
            right: right,
        },
        PhantomData,
    )
}

#[derive(Clone, Debug)]
pub struct And<LEFT: Operation<bool>, RIGHT: Operation<bool>> {
    pub left: LEFT,
    pub right: RIGHT,
}

impl<LEFT: Operation<bool>, RIGHT: Operation<bool>> Operation<bool> for And<LEFT, RIGHT> {
    fn evaluate(&self, context: &mut DifferentiatedCPUContext) -> bool {
        self.left.evaluate(context) && self.right.evaluate(context)
    }
}

//impl<R: Computable, LEFT: Differentiable<R>, RIGHT: Differentiable<R>> Differentiable<bool> for And<R, LEFT, RIGHT> {
//    type Diff = Self;
//
//    fn auto_diff_for<R1: Computable>(&self, _var: super::var::Variable<R1>) -> Self::Diff {
//        return self.clone()
//    }
//    fn contains_var<R1: Computable>(&self, var: super::var::Variable<R1>) -> bool {
//        self.left.contains_var(var.clone()) || self.right.contains_var(var)
//    }
//}
