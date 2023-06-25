use std::marker::PhantomData;

use crate::core::{types::Computable, operation::{Operation, OperationWrapper, Differentiable}, type_traits::{IndexAble, GetAndSetable}, processor::{cpu::DifferentiatedCPUContext}};

pub fn index<R: Computable, T: IndexAble, O: Operation<u32>>(tensor: &T, op: O) -> OperationWrapper<R, Index<R, T, O>> {
    OperationWrapper (
        Index {
            tensor: tensor.clone(),
            index: op,
            _0: PhantomData,
        },
        PhantomData,
    )
}


#[derive(Clone, Debug)]
pub struct Index<R: Computable, T: IndexAble, O: Operation<u32>> {
    pub tensor: T,
    pub index: O,
    _0: PhantomData<R>,
}

impl<R: Computable, T: IndexAble, O: Operation<u32>> Operation<R> for Index<R, T, O> {
    fn evaluate(&self, _context: &mut DifferentiatedCPUContext) -> R {
        todo!("Unfinished business")
    }
}

impl<R: Computable, T: IndexAble, O: Differentiable<u32>> Differentiable<R> for Index<R, T, O> {
    type Diff = R;

    fn auto_diff_for<R1: Clone>(&self, var: super::var::Variable<R1>, _var_trace: &mut std::collections::HashMap<String, Vec<String>>) -> Self::Diff {
        if self.contains_var(var) {
            R::from_int(1)
        } else {
            R::get_zero()
        }
    }

    fn contains_var<R1: Clone>(&self, var: super::var::Variable<R1>) -> bool {
        if let Some(refr) = self.get_variable() {
            refr.split(".").next().unwrap() == var.reference
        } else {
            false
        }
    }
}

impl<R: Computable, T: IndexAble, O: Operation<u32>> GetAndSetable<R> for Index<R, T, O> {
    fn get_variable(&self) -> Option<String> {
        Some(self.tensor.get_reference())
    }
}