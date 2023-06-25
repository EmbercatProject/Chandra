use std::{marker::PhantomData, collections::HashMap};

use crate::core::{
    operation::{Operation, OperationWrapper, Differentiable},
    types::{Void, Value, Either}, processor::cpu::DifferentiatedCPUContext,
};

use super::{scope::Scope, noop::Noop};

pub fn if_then<CONDITION: Operation<bool>, A: Operation<Void>, B: Operation<Void>>(
    condition: CONDITION,
    then: Scope<Void, Void, Void, A, B>,
) -> OperationWrapper<Void, IfElse<Void, CONDITION, A, B>> {
    OperationWrapper(
        IfElse {
            condition,
            then,
            els: None
        },
        PhantomData,
    )
}

pub fn if_then_or_else<R: Value, CONDITION: Operation<bool>, A: Operation<R>, B: Operation<Void>, C: Operation<R>, D: Operation<Void>>(
    condition: CONDITION,
    then: Scope<R, R, Void, A, B>,
    els: Scope<R, R, Void, C, D>
) -> OperationWrapper<R, IfElse<R, CONDITION, A, B, C, D>> {
    OperationWrapper(
        IfElse {
            condition,
            then,
            els: Some(els)
        },
        PhantomData,
    )
}

#[derive(Clone, Debug)]
pub struct IfElse<R: Value, CONDITION: Operation<bool>, A: Operation<R>, B: Operation<Void>, C: Operation<R> = Noop, D: Operation<Void> = Noop> {
    pub condition: CONDITION,
    pub then: Scope<R, R, Void, A, B>,
    pub els: Option<Scope<R, R, Void, C, D>>
}

impl<R: Value, CONDITION: Operation<bool>, A: Operation<R>, B: Operation<Void>, C: Operation<R>, D: Operation<Void>> Operation<R> for IfElse<R, CONDITION, A, B, C, D> {
    fn evaluate(&self, context: &mut DifferentiatedCPUContext) -> R {
        if self.condition.evaluate(context) {
            self.then.evaluate(context)
        } else if let Some(els) = &self.els {
            els.evaluate(context)
        } else {
            todo!("Figure out how to return R")
        }
    }
}

impl<R: Value, CONDITION: Operation<bool>, A: Differentiable<R>, B: Differentiable<Void>, C: Differentiable<R>, D: Differentiable<Void>> Differentiable<R> for IfElse<R, CONDITION, A, B, C, D> {
    type Diff = IfElse<R, CONDITION, Either<A::Diff, A>, Either<B::Diff, B>, Either<C::Diff, C>, Either<D::Diff, D>>;

    fn auto_diff_for<R1: Clone>(&self, var: super::var::Variable<R1>, var_trace: &mut HashMap<String, Vec<String>>) -> Self::Diff {
        let a = self.then.auto_diff_for(var.clone(), var_trace);

        let b = if let Some(els) = &self.els {
            Some(els.auto_diff_for(var.clone(), var_trace))
        } else {
           None
        };

        IfElse { condition: self.condition.clone(), then: a, els: b }
    }

    fn contains_var<R1: Clone>(&self, var: super::var::Variable<R1>) -> bool {
        self.then.contains_var(var.clone()) || if let Some(els) = &self.els {els.contains_var(var)} else {false}
    }
}
