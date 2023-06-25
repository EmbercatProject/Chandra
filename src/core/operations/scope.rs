use std::{marker::PhantomData, collections::HashMap};

use crate::core::{operation::{Operation, Differentiable}, types::{Void, Value, Either}, processor::cpu::DifferentiatedCPUContext};

use super::{instruction_list::InstructionList, noop::Noop};

pub trait ScopeTrait {
    //type This<R: Value, I1: Operation<R>, I2: Operation<Void>>;
    type A;
    type B;

    //type This<R: Value, I1: Operation<R>, I2: Operation<Void>> = Scope<R, I1, I2>;
}

impl<R: Value, R1: Value, R2: Value, A: Operation<R1>, B: Operation<R2>> ScopeTrait for Scope<R, R1, R2, A, B> {
    type A = A;
    type B = B;
}

#[derive(Clone, Debug)]
pub struct Scope<R: Value, R1: Value, R2: Value, A: Operation<R1>, B: Operation<R2>> {
    pub instruction: InstructionList<R1, R2, A, B>,
    _0: PhantomData<R>
}

impl Scope<Void, Void, Void, Noop, Noop> {
    pub fn new<R: Value>() -> Scope<R, Void, Void, Noop, Noop> {
        return Scope {instruction: InstructionList::new(Noop), _0: PhantomData}
    }
}

impl<R: Value, R1: Value, R2: Value, A: Operation<R1>, B: Operation<R2>> Scope<R, R1, R2, A, B> {
    pub fn include<I: Operation<Void>>(self, instr: I) -> Scope<R, Void, R1, I, InstructionList<R1, R2, A, B>> {
        let instruction = self.instruction.append(instr);
        return Scope {instruction: instruction, _0: PhantomData}
    }

    pub fn returns<I: Operation<R>>(self, instr: I) -> Scope<R, R, R1, I, InstructionList<R1, R2, A, B>> {
        let instruction = self.instruction.append(instr);
        return Scope {instruction: instruction, _0: PhantomData}
    }
} 

impl<R: Value, R2: Value, A: Operation<R>, B: Operation<R2>> Operation<R> for Scope<R, R, R2, A, B> {
    fn evaluate(&self, context: &mut DifferentiatedCPUContext) -> R {
        self.instruction.evaluate(context)
    }
}

impl<R: Value, R2: Value, A: Differentiable<R>, B: Differentiable<R2>> Differentiable<R> for Scope<R, R, R2, A, B> {
    type Diff = Scope<R, R, R2, Either<A::Diff, A>, Either<B::Diff, B>>;

    fn auto_diff_for<R1: Clone>(&self, var: super::var::Variable<R1>, var_trace: &mut HashMap<String, Vec<String>>) -> Self::Diff {
        Scope { instruction: self.instruction.auto_diff_for(var, var_trace), _0: PhantomData }
    }
    fn contains_var<R1: Clone>(&self, var: super::var::Variable<R1>) -> bool {
        self.instruction.contains_var(var.clone())
    }
}