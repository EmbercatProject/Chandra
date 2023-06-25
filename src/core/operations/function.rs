use std::marker::PhantomData;

use crate::core::{
    operation::{Operation, Differentiable},
    types::{Void, Value},type_traits::{FunctionInputs},
};

use super::{scope::{Scope}};

pub fn function<R: Value, INPUTS: FunctionInputs, A: Operation<R>, B: Operation<Void>, F>(
    name: &str,
    inputs: INPUTS,
    function: F,
) -> Function<R, INPUTS, Scope<R, R, Void, A, B>>  
    where F: FnOnce(INPUTS) -> Scope<R, R, Void, A, B>
    {
    let scope = function(inputs.clone());

        Function {
            name: name.to_string(),
            inputs,
            scope,
            _0: PhantomData
        }
}

#[derive(Clone, Debug)]
pub struct Function<R: Value, INPUTS: FunctionInputs, A: Operation<R>> {
    pub name: String,
    pub inputs: INPUTS,
    pub scope: A,
    _0: PhantomData<R>
}

impl<R: Value, INPUTS: FunctionInputs, A: Operation<R>> Operation<R> for Function<R, INPUTS, A> {
    fn evaluate(&self, _context: &mut crate::core::processor::cpu::DifferentiatedCPUContext) -> R {
        todo!("Unfinished business")
    }
}

impl<R: Value, INPUTS: FunctionInputs, A: Differentiable<R>> Differentiable<R> for Function<R, INPUTS, A> {
    type Diff = Function<R, INPUTS, A::Diff>;

    fn auto_diff_for<R1: Clone>(&self, var: super::var::Variable<R1>, var_trace: &mut std::collections::HashMap<String, Vec<String>>) -> Self::Diff {
        Function {
            name: format!("{}_chandra_diff", self.name),
            inputs: self.inputs.clone(),
            scope: self.scope.auto_diff_for(var, var_trace),
            _0: PhantomData,
        }
    }

    fn contains_var<R1: Clone>(&self, var: super::var::Variable<R1>) -> bool {
        self.scope.contains_var(var)
    }
}

