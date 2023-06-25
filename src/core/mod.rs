use std::{collections::HashMap};

use self::{allocated::{ExecutableBindings}, operation::{Operation, Compilable, Differentiable}, types::{Void}, processor::{cpu::{CPUFunction}, ProcessorInformation}, operations::var::Variable};
use crate::{processor::cpu::{CPUStorage, CPUProcessor}, types::differential::Differential};

pub mod operation;
pub mod operations;
pub mod processor;
pub mod types;
pub mod type_traits;
pub mod any_map;
pub mod allocated;
pub mod guards;

pub trait Buildable<P: ProcessorInformation> {
    type Binding: ExecutableBindings<P::Storage>;
    type CPUBinding: ExecutableBindings<CPUStorage>;
    type CPUFunction: CPUFunction<Self::CPUBinding>;
    type Main: Operation<Void> + Compilable<Void, P::Compiler>;

    fn get_cpu(&self) -> Self::CPUFunction;
    fn get_main_tree(&self) -> Self::Main;
}

pub trait DifferentiableProgram<P: Program> where <P as Program>::MainTree: Differentiable<Void> {
    fn differantiate_for<R: Clone>(&self, var: Variable<R>) -> Differential<P>;
}

impl<P: Program> DifferentiableProgram<P> for P where <P as Program>::MainTree: Differentiable<Void> {
    fn differantiate_for<R: Clone>(&self, var: Variable<R>) -> Differential<P> {
        let tree: <P as Program>::MainTree = <P as Program>::get_main_tree(&self);
        let mut trace = HashMap::new();
        
        Differential {
            main: tree.auto_diff_for(var, &mut trace),
            cpu_fn: <P as Buildable<CPUProcessor>>::get_cpu(&self),
        } 
    }
}

pub trait Program: Buildable<CPUProcessor> {
    type MainTree: Operation<Void>;

    fn get_main_tree(&self) -> Self::MainTree;
}