use std::marker::PhantomData;

use crate::core::{types::{Computable, Void}, operation::{Operation, OperationWrapper, Differentiable}, type_traits::GetAndSetable, processor::cpu::DifferentiatedCPUContext};



pub fn set<R: Computable, G: GetAndSetable<R>, O: Operation<R>>(var: &G, assign: O) -> OperationWrapper<Void, Set<R, G, O>> {
    OperationWrapper(
        Set {
            getable: var.clone(),
            assign,
            _0: PhantomData
        }, 
        PhantomData
    )
}

#[derive(Clone, Debug)]
pub struct Set<R: Computable, G: GetAndSetable<R>, O: Operation<R>> {
    pub getable: G,
    pub assign: O,
    _0: PhantomData<R>
}

impl<R: Computable, G: GetAndSetable<R>, O: Operation<R>> Operation<Void> for Set<R, G, O> {
    fn evaluate(&self, _context: &mut DifferentiatedCPUContext) -> Void {
        if let Some(_var) = self.getable.get_variable() {
            todo!()
            //context.set(&var, self.assign.evaluate(context));
        } else {
            panic!("Unexpected")
        }
        
        //Void
    }
}

impl<R: Computable, G: GetAndSetable<R>, O: Differentiable<R>> Differentiable<Void> for Set<R, G, O> {
    type Diff = OperationWrapper<Void, Set<R, G, O::Diff>>;

    fn auto_diff_for<R1: Clone>(&self, var: super::var::Variable<R1>, var_trace: &mut std::collections::HashMap<String, Vec<String>>) -> Self::Diff {
        if let Some(refr) = self.getable.get_variable() {
            if self.assign.contains_var(var.clone()) {
                if let Some (trace) = var_trace.get_mut(&refr) {
                    trace.push(var.reference.clone());
                } else {
                    var_trace.insert(refr, vec![var.reference.clone()]);
                }   
            }

            set(&self.getable, self.assign.auto_diff_for(var, var_trace))
        } else {
            panic!("Internal Differentiation Error")
        }
    }

    fn contains_var<R1: Clone>(&self, var: super::var::Variable<R1>) -> bool {
        self.assign.contains_var(var.clone()) ||
            if let Some(refr) = self.getable.get_variable() {
                refr == var.reference 
            } else { false }
    }
}