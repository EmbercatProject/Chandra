use std::{marker::PhantomData, collections::HashMap};

use crate::core::{
    operation::{Operation, OperationWrapper, Differentiable},
    processor::cpu::DifferentiatedCPUContext, type_traits::Calculatable, 
};

use super::{multiply::{Multiply, multiply}, subtract::{Subtract, subtract}};

pub fn divide<R: Calculatable, LEFT: Operation<R>, RIGHT: Operation<R>>(
    left: LEFT,
    right: RIGHT,
) -> OperationWrapper<R, Divide<R, LEFT, RIGHT>> {
    OperationWrapper(
        Divide {
            left: left,
            right: right,
            _0: PhantomData,
        },
        PhantomData,
    )
}

#[derive(Clone, Debug)]
pub struct Divide<R: Calculatable, LEFT: Operation<R>, RIGHT: Operation<R>> {
    pub left: LEFT,
    pub right: RIGHT,
    _0: PhantomData<R>,
}

impl<R: Calculatable, LEFT: Operation<R>, RIGHT: Operation<R>> Operation<R> for Divide<R, LEFT, RIGHT> {
    fn evaluate(&self, context: &mut DifferentiatedCPUContext) -> R {
       self.left.evaluate(context) / self.right.evaluate(context)
    }
}


impl<R: Calculatable, LEFT: Differentiable<R>, RIGHT: Differentiable<R>> Differentiable<R> for Divide<R, LEFT, RIGHT> {
    type Diff = OperationWrapper<R, Divide<R, OperationWrapper<R, Subtract<R, OperationWrapper<R, Multiply<R, LEFT::Diff, RIGHT>>, OperationWrapper<R, Multiply<R, LEFT, RIGHT::Diff>>>>, OperationWrapper<R, Multiply<R, RIGHT, RIGHT>>>>;

    fn auto_diff_for<R1: Clone>(&self, var: super::var::Variable<R1>, var_trace: &mut HashMap<String, Vec<String>>) -> Self::Diff {
        divide(subtract(multiply(self.left.auto_diff_for(var.clone(), var_trace), self.right.clone()), multiply(self.left.clone(), self.right.auto_diff_for(var.clone(), var_trace))), multiply(self.right.clone(), self.right.clone()))
    }
    fn contains_var<R1: Clone>(&self, var: super::var::Variable<R1>) -> bool {
        self.left.contains_var(var.clone()) || self.right.contains_var(var)
    }
}