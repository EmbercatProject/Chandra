use std::{marker::PhantomData, collections::HashMap};
use std::fmt::Debug;
use super::processor::cpu::DifferentiatedCPUContext;
use super::{types::{Value, Computable, Void}, type_traits::{IndexAble, GetAndSetable}, operations::var::Variable};

pub trait Operation<R: Value>: Clone + Debug {
    fn evaluate(&self, context: &mut DifferentiatedCPUContext) -> R;
}

pub trait Differentiable<R: Value>: Operation<R> {
    type Diff: Operation<R>;

    fn auto_diff_for<R1: Clone>(&self, var: Variable<R1>, var_trace: &mut HashMap<String, Vec<String>>) -> Self::Diff;
    fn contains_var<R1: Clone>(&self, var: Variable<R1>) -> bool;
}

pub trait Compilable<R: Value, C> {
    fn build(&self, compiler: &mut C);
}

impl<R: Value, O: Operation<R>> Compilable<R, Void> for O {
    fn build(&self, _compiler: &mut Void) {
        todo!()
    }
}


#[derive(Clone, Debug)]
pub struct OperationWrapper<R: Value, O: Operation<R>> (pub O, pub PhantomData<R>);

impl<R: Value, O: Operation<R>> Operation<R> for OperationWrapper<R, O> {
    fn evaluate(&self, context: &mut DifferentiatedCPUContext) -> R {
        self.0.evaluate(context)
    }
}

impl<R: Value, O: Differentiable<R>> Differentiable<R> for OperationWrapper<R, O> {
    type Diff = O::Diff;

    fn auto_diff_for<R1: Clone>(&self, var: Variable<R1>, var_trace: &mut HashMap<String, Vec<String>>) -> Self::Diff {
        self.0.auto_diff_for(var, var_trace)
    }

    fn contains_var<R1: Clone>(&self, var: Variable<R1>) -> bool {
        self.0.contains_var(var)
    }
}

impl<R: Value, I: Operation<R>> IndexAble for OperationWrapper<R, I> where I: IndexAble {
    type IndexResult = I::IndexResult;

    fn index<O: Operation<u32>>(&self, _index: O) -> Self::IndexResult {
        todo!()
    }

    fn get_reference(&self) -> String {
        self.0.get_reference()
    }

    fn get_field() -> Option<String> {
        I::get_field()
    }
    
}

impl<R: Computable, G: Operation<R>> GetAndSetable<R> for OperationWrapper<R, G> where G: GetAndSetable<R> {
    fn get_variable(&self) -> Option<String> {
        self.0.get_variable()
    }
}