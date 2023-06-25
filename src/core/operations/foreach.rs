use std::{marker::PhantomData, collections::HashMap};

use crate::core::{
    operation::{Operation, OperationWrapper, Differentiable},
    types::{Computable, Void, Either},type_traits::Iterable, processor::cpu::DifferentiatedCPUContext,
};

use super::{scope::Scope, var::Variable};

pub fn foreach<R: Computable, ITERABLE: Iterable<R>, A: Operation<Void>, B: Operation<Void>, F>(
    iterable: ITERABLE,
    function: F,
) -> OperationWrapper<Void, ForEach<R, ITERABLE, A, B>> 
    where F: FnOnce(Variable<R>) -> Scope<Void, Void, Void, A, B>
{
    let loop_var = Variable::<R>::new("loop_variable");

    let scope = function(loop_var);
    OperationWrapper(
        ForEach {
            iterable,
            scope,
            _0: PhantomData,
        },
        PhantomData,
    )
}

#[derive(Clone, Debug)]
pub struct ForEach<R: Computable, ITERABLE: Iterable<R>, A: Operation<Void>, B: Operation<Void>> {
    pub iterable: ITERABLE,
    pub scope: Scope<Void, Void, Void, A, B>,
    _0: PhantomData<R>,
}

impl<R: Computable, ITERABLE: Iterable<R>, A: Operation<Void>, B: Operation<Void>> Operation<Void> for ForEach<R, ITERABLE, A, B> {
    fn evaluate(&self, _context: &mut DifferentiatedCPUContext) -> Void {
        todo!("Unfinished business")
        //self.left.evaluate(context) / self.right.evaluate(context)
     }
}

impl<R: Computable, ITERABLE: Iterable<R>, A: Differentiable<Void>, B: Differentiable<Void>> Differentiable<Void> for ForEach<R, ITERABLE, A, B> {
    type Diff = ForEach<R, ITERABLE, Either<A::Diff, A>, Either<B::Diff, B>>;

    fn auto_diff_for<R1: Clone>(&self, var: Variable<R1>, var_trace: &mut HashMap<String, Vec<String>>) -> Self::Diff {
        ForEach {
            iterable: self.iterable.clone(),
            scope: self.scope.auto_diff_for(var, var_trace),
            _0: PhantomData,
        }
    }

    fn contains_var<R1: Clone>(&self, var: Variable<R1>) -> bool {
        self.scope.contains_var(var)
    }
}
