use std::marker::PhantomData;

use crate::core::{types::{Computable, Void}, operation::{Operation, OperationWrapper, Differentiable}, processor::cpu::DifferentiatedCPUContext};

use super::var::Variable;

pub fn assign<R: Computable, O: Operation<R>>(name: String, operation: O) -> (Variable<R>, OperationWrapper<Void, Assign<R,O>>) {
    let variable = Variable::<R>::new(&name);
    let instruction = OperationWrapper(Assign {
        variable: variable.clone(),
        assign: operation
    },
    PhantomData);

    return (variable, instruction) 
}

#[derive(Clone, Debug)]
pub struct Assign<R: Computable, O: Operation<R>> {
    pub variable: Variable<R>,
    pub assign: O
}

impl<R: Computable, O: Operation<R>> Operation<Void> for Assign<R, O> {
    fn evaluate(&self, _context: &mut DifferentiatedCPUContext) -> Void {
        todo!()
       //let a = self.assign.evaluate(context);
       //context.set::<R>(&self.variable.reference, a);
       //Void
    }
}

impl<R: Computable, O: Differentiable<R>> Differentiable<Void> for Assign<R, O> {
    type Diff = OperationWrapper<Void, Assign<R, O::Diff>>;

    fn auto_diff_for<R1: Clone>(&self, var: Variable<R1>, var_trace: &mut std::collections::HashMap<String, Vec<String>>) -> Self::Diff {
        if self.assign.contains_var(var.clone()) {
            var_trace.insert(self.variable.reference.clone(), vec![var.reference.clone()]);
        } else {
            var_trace.insert(self.variable.reference.clone(), vec![]);
        }

        assign(self.variable.reference.clone(), self.assign.auto_diff_for(var, var_trace)).1
    }

    fn contains_var<R1: Clone>(&self, var: Variable<R1>) -> bool {
        self.assign.contains_var(var)
    }
}