
use crate::{core::{operation::{Differentiable, Compilable}, types::Void, processor::ProcessorInformation, Buildable, Program}, processor::cpu::CPUProcessor};

#[derive(Clone, Debug)]
pub struct Differential<P: Program> where P::MainTree: Differentiable<Void> {
    pub main:< <P as Program>::MainTree as Differentiable<Void>>::Diff,
    pub cpu_fn: <P as Buildable<CPUProcessor>>::CPUFunction,
}

impl<P: ProcessorInformation , B: Buildable<P> + Program> Buildable<P> for Differential<B> where
    <B as Program>::MainTree: Differentiable<Void>,
    <<B as Program>::MainTree as Differentiable<Void>>::Diff: Compilable<Void, P::Compiler> {
        type Binding = <B as Buildable<P>>::Binding;
        type CPUBinding = <B as Buildable<CPUProcessor>>::CPUBinding;
        type CPUFunction = <B as Buildable<CPUProcessor>>::CPUFunction;
    
        type Main = <<B as Program>::MainTree as Differentiable<Void>>::Diff;
    
        fn get_cpu(&self) -> Self::CPUFunction {
            self.cpu_fn.clone()
        }
    
        fn get_main_tree(&self) -> Self::Main {
            self.main.clone()
        }
    }