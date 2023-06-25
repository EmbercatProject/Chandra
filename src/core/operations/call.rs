use std::marker::PhantomData;

use crate::core::{
    operation::{Operation, OperationWrapper, Differentiable},
    types::{Value, Either, Computable}, type_traits::{ FunctionInputs, CallInputs, CallMatchFunctionInputs, DiffableFunctionInputs}, processor::cpu::DifferentiatedCPUContext,
};

use super::{function::Function};

pub fn call<R: Value, INPUTS: FunctionInputs, CALLINPUTS: CallInputs + CallMatchFunctionInputs<INPUTS>, A: Operation<R>> (
    inputs: CALLINPUTS,
    function: Function<R, INPUTS, A>,
) -> OperationWrapper<R, Call<R, INPUTS, CALLINPUTS, A>> {
    OperationWrapper(
        Call {
            inputs,
            function,
        },
        PhantomData,
    )
}

#[derive(Clone, Debug)]
pub struct Call<R: Value, INPUTS: FunctionInputs, CALLINPUTS: CallInputs + CallMatchFunctionInputs<INPUTS>, A: Operation<R>> {
    pub inputs: CALLINPUTS,
    pub function: Function<R, INPUTS, A>,
}

impl<R: Value, INPUTS: FunctionInputs, CALLINPUTS: CallInputs + CallMatchFunctionInputs<INPUTS>, A: Operation<R>> Operation<R> for Call<R, INPUTS, CALLINPUTS, A> {
    fn evaluate(&self, _context: &mut DifferentiatedCPUContext) -> R {
       //
       todo!("Unfinished business");
       //context.set_return_state(false);
     }
}

impl<R: Computable, INPUTS: FunctionInputs, CALLINPUTS: CallInputs + CallMatchFunctionInputs<INPUTS> + DiffableFunctionInputs<R, INPUTS>, A: Differentiable<R>> Differentiable<R> for Call<R, INPUTS, CALLINPUTS, A> {
    type Diff = Either<CALLINPUTS::Diff<A>, Call<R, INPUTS, CALLINPUTS, A>>;

    fn auto_diff_for<R1: Clone>(&self, var: super::var::Variable<R1>, var_trace: &mut std::collections::HashMap<String, Vec<String>>) -> Self::Diff {
        match self.inputs.map_variable(var.clone(), self.function.inputs.clone()) {
            Some(mapped) => {
                Either::A(self.inputs.auto_diff_for(self.clone(), mapped, var.clone(), var_trace))
            }
            None => {
                Either::B(Call {
                    inputs: self.inputs.clone(),
                    function: self.function.clone(),
                })
            }
        }
    }

    fn contains_var<R1: Clone>(&self, var: super::var::Variable<R1>) -> bool {
        self.inputs.contains_var(var)
    }
}
