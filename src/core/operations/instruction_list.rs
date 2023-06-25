use std::{marker::PhantomData, collections::HashMap};

use crate::core::{types::{Void, Value, Either}, operation::{Operation, Differentiable}, processor::cpu::DifferentiatedCPUContext};

#[derive(Clone, Debug)]
pub struct InstructionList<R1: Value, R2: Value, A: Operation<R1>, B: Operation<R2>> {
    pub this: A,
    pub previous: Option<B>,
    _0: PhantomData<R1>,
    _1: PhantomData<R2>,
}

impl<A: Operation<Void>> InstructionList<Void, Void, A, A> {
    pub fn new(instr: A) -> Self {
        return Self {this: instr, previous: None, _0: PhantomData, _1: PhantomData}
    }
}

impl<R1: Value, R2: Value, A: Operation<R1>, B: Operation<R2>> InstructionList<R1, R2, A, B> {
    pub fn append<R: Value, C: Operation<R>>(self, instr: C) -> InstructionList<R, R1, C, Self> {
        return InstructionList::<R, R1, C, InstructionList<R1, R2, A, B>> {this: instr, previous: Some(self), _0: PhantomData, _1: PhantomData}
    }
}

impl<R1: Value, R2: Value, A: Operation<R1>, B: Operation<R2>> Operation<R1> for InstructionList<R1, R2, A, B> {
    fn evaluate(&self, context: &mut DifferentiatedCPUContext) -> R1 {
        if let Some(previous) = &self.previous {
            let _prev = previous.evaluate(context);

            if context.is_in_return_state() {
                todo!("fix instruction list with Either type") //prev;
            }
        }
        self.this.evaluate(context)
     }
}

impl<R1: Value, R2: Value, A: Differentiable<R1>, B: Differentiable<R2>> Differentiable<R1> for InstructionList<R1, R2, A, B> {
    type Diff = InstructionList<R1, R2, Either<A::Diff, A>, Either<B::Diff, B>>;

    fn auto_diff_for<R:Clone>(&self, var: super::var::Variable<R>, var_trace: &mut HashMap<String, Vec<String>>) -> Self::Diff {
        let b = if let Some(prev) = &self.previous {
            if prev.contains_var(var.clone()) {
                Some(Either::A(prev.auto_diff_for(var.clone(), var_trace)))
            } else {
                Some(Either::B(prev.clone()))
            }
        } else {
           None
        };

        let a = if self.this.contains_var(var.clone()) {
            Either::A(self.this.auto_diff_for(var.clone(), var_trace))
        } else {
            Either::B(self.this.clone())
        };

        InstructionList {
            this: a,
            previous: b,
            _0: PhantomData,
            _1: PhantomData,
        }
    }

    fn contains_var<R:Clone>(&self, var: super::var::Variable<R>) -> bool {
        self.this.contains_var(var.clone()) || if let Some(x) = &self.previous {
            x.contains_var(var)
        } else {false}
    }
}
