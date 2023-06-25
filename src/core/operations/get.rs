use std::{marker::PhantomData, collections::HashMap};

use crate::core::{types::{Computable, Either}, operation::{Operation, OperationWrapper, Differentiable}, processor::cpu::DifferentiatedCPUContext};

use super::var::Variable;

pub fn get<R: Computable>(var: &Variable<R>) -> OperationWrapper<R, Get<R>> {
    OperationWrapper(
        Get {
            getable: var.clone()
        }, 
        PhantomData
    )
}

#[derive(Clone, Debug)]
pub struct Get<R: Computable> {
    pub getable: Variable<R>
}

impl<R: Computable> Operation<R> for Get<R> {
    fn evaluate(&self, _context: &mut DifferentiatedCPUContext) -> R {
        todo!()
     }
}

impl<R: Computable> Differentiable<R> for Get<R> {
    type Diff = Either<OperationWrapper<R, Get<R>>, R>;

    fn auto_diff_for<R1: Clone>(&self, var: Variable<R1>, var_trace: &mut HashMap<String, Vec<String>>) -> Self::Diff {
        let inner = self.getable.auto_diff_for(var, var_trace);
        match inner {
           Either::A(v) => Either::A(get(&v)),
           Either::B(r) => Either::B(r),
        }
    }

    fn contains_var<R1: Clone>(&self, var: Variable<R1>) -> bool {
        self.getable.reference == var.reference
    }
}