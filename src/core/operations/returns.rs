use std::marker::PhantomData;

use crate::core::{types::{Value}, operation::{Operation, Differentiable}, processor::cpu::DifferentiatedCPUContext};



pub fn returns<R: Value, O: Operation<R>>(operation: O) -> Returns<R, O> {
    Returns { operation, _0: PhantomData }
}

#[derive(Clone, Debug)]
pub struct Returns<R: Value, O: Operation<R>> {
    pub operation: O,
    _0: PhantomData<R>
}

impl<R: Value, O: Operation<R>> Operation<R> for Returns<R, O> {
    fn evaluate(&self, context: &mut DifferentiatedCPUContext) -> R {
        context.set_return_state(true);
        self.operation.evaluate(context)
        //self.operation.evaluate(context)
    }
}

impl<R: Value, O: Differentiable<R>> Differentiable<R> for Returns<R, O> {
    type Diff = Returns<R, O::Diff>;

    fn auto_diff_for<R1: Clone>(&self, var: super::var::Variable<R1>, var_trace: &mut std::collections::HashMap<String, Vec<String>>) -> Self::Diff {
        returns(self.operation.auto_diff_for(var, var_trace))
    }

    fn contains_var<R1: Clone>(&self, var: super::var::Variable<R1>) -> bool {
        self.operation.contains_var(var)
    }
}