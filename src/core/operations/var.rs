use std::{marker::PhantomData, collections::HashMap};

use crate::core::{types::{Computable, Either}, operation::{Operation, Differentiable}, type_traits::{GetAndSetable, IndexAble}, processor::cpu::DifferentiatedCPUContext};

#[derive(Clone, Debug)]
pub struct Variable<R: Clone> {
    pub reference: String,
    pub(crate) _0: PhantomData<R>
}

impl<R: Clone> Variable<R> {
    pub fn new(name: &str) -> Self {
        Variable {
            reference: name.into(),
            _0: PhantomData
        }
    }
}

impl<R: Computable> Operation<R> for Variable<R> {
    fn evaluate(&self, _context: &mut DifferentiatedCPUContext) -> R {
        unimplemented!("Probably never called?")
     }
}

impl<R: Computable> Differentiable<R> for Variable<R> {
    type Diff = Either<Variable<R>, R>;

    fn auto_diff_for<R1: Clone>(&self, var: Variable<R1>, var_trace: &mut HashMap<String, Vec<String>>) -> Self::Diff {
        if self.reference == var.reference {
            Either::B(R::from_int(1))
        } else if let Some(trace) = var_trace.get(&self.reference) {
            if trace.iter().any(|t| t == &var.reference) {
                Either::A(self.clone())
            } else {
                Either::B(R::from_int(0))
            }
        } else {
            Either::B(R::from_int(0))
        }
    }

    fn contains_var<R1: Clone>(&self, var: Variable<R1>) -> bool {
        self.reference == var.reference
    }
}

impl<R: Computable> GetAndSetable<R> for Variable<R> {
    fn get_variable(&self) -> Option<String> {
        Some(self.reference.clone())
    }
}

impl<I: IndexAble> IndexAble for Variable<I> {
    type IndexResult = I::IndexResult;

    fn index<O: Operation<u32>>(&self, _index: O) -> Self::IndexResult {
        todo!()
    }

    fn get_reference(&self) -> String {  
        if let Some(x) = I::get_field() {
            format!("{}.{}", self.reference.clone(), x)
        } else {
            self.reference.clone()
        }   
    }

    fn get_field() -> Option<String> {
        I::get_field()
    }
}