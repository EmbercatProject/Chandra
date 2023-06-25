use std::marker::PhantomData;

use crate::core::{
    operation::{Operation, OperationWrapper},
    types::{Void}, processor::cpu::DifferentiatedCPUContext,
};

use super::{scope::Scope};

pub fn until<CONDITION: Operation<bool>, A: Operation<Void>, B: Operation<Void>>(
    condition: CONDITION,
    scope: Scope<Void, Void, Void, A, B>,
) -> OperationWrapper<Void, Until<CONDITION, A, B>> {
    
    OperationWrapper(
        Until {
            condition,
            scope,
        },
        PhantomData,
    )
}

#[derive(Clone, Debug)]
pub struct Until<CONDITION: Operation<bool>, A: Operation<Void>, B: Operation<Void>> {
    pub condition: CONDITION,
    pub scope: Scope<Void, Void, Void, A, B>,
}

impl<CONDITION: Operation<bool>, A: Operation<Void>, B: Operation<Void>> Operation<Void> for Until<CONDITION, A, B> {
    fn evaluate(&self, _context: &mut DifferentiatedCPUContext) -> Void {
        todo!()
    }
}
