use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

use super::{types::{Computable, Value}, operation::{Operation, OperationWrapper, Differentiable}, processor::Storage, allocated::{MemoryLayoutDescriptor, ParallelizationDescriptor}, operations::{var::Variable, call::Call, multiply::{Multiply, multiply}}};

pub trait IndexAble: Clone + Debug {
    type IndexResult;

    fn index<O: Operation<u32>>(&self, index: O) -> Self::IndexResult;

    fn get_reference(&self) -> String;

    fn get_field() -> Option<String>;
}

pub trait GetAndSetable<R: Computable>: Clone + Debug {
    fn get_variable(&self) -> Option<String>;
}

pub trait Iterable<R: Computable>: Operation<R> + Clone + Debug {
    type StartOp: Operation<R>;
    type NextOp: Operation<R>;
    type BoundryOp: Operation<R>;

    fn get_start(&self) -> Self::StartOp;
    fn get_next(&self) -> Self::NextOp;
    fn get_boundry(&self) -> Self::BoundryOp;
}

impl<R: Computable, O: Operation<R> + Iterable<R>> Iterable<R> for OperationWrapper<R, O> {
    type StartOp = O::StartOp;

    type NextOp = O::NextOp;

    type BoundryOp = O::BoundryOp;

    fn get_start(&self) -> Self::StartOp {
        self.0.get_start()
    }

    fn get_next(&self) -> Self::NextOp {
        self.0.get_next()
    }

    fn get_boundry(&self) -> Self::BoundryOp {
        self.0.get_boundry()
    }
}

pub trait ChandraFn {
    type Result: Value;
    type Inputs: FunctionInputs;
    type FunctionScope: Operation<Self::Result>;
}

pub trait ChandraExtensionFn {
    type Result: Value;
    type Inputs: FunctionInputs;
    type FunctionScope: Operation<Self::Result>;
}

pub trait FunctionInputs: Clone + Debug {}

impl<R: Computable> FunctionInputs for (Variable<R>,) {}
impl<R: Computable, R2: Computable> FunctionInputs for (Variable<R>, Variable<R2>) {}
impl<R: Computable, R2: Computable, R3: Computable> FunctionInputs for (Variable<R>, Variable<R2>, Variable<R3>) {}
impl<R: Computable, R2: Computable, R3: Computable, R4: Computable> FunctionInputs for (Variable<R>, Variable<R2>, Variable<R3>, Variable<R4>) {}
impl<R: Computable, R2: Computable, R3: Computable, R4: Computable, R5: Computable> FunctionInputs for (Variable<R>, Variable<R2>, Variable<R3>, Variable<R4>, Variable<R5>) {}
impl<R: Computable, R2: Computable, R3: Computable, R4: Computable, R5: Computable, R6: Computable> FunctionInputs for (Variable<R>, Variable<R2>, Variable<R3>, Variable<R4>, Variable<R5>, Variable<R6>) {}
impl<R: Computable, R2: Computable, R3: Computable, R4: Computable, R5: Computable, R6: Computable, R7: Computable> FunctionInputs for (Variable<R>, Variable<R2>, Variable<R3>, Variable<R4>, Variable<R5>, Variable<R6>, Variable<R7>) {}
impl<R: Computable, R2: Computable, R3: Computable, R4: Computable, R5: Computable, R6: Computable, R7: Computable, R8: Computable> FunctionInputs for (Variable<R>, Variable<R2>, Variable<R3>, Variable<R4>, Variable<R5>, Variable<R6>, Variable<R7>, Variable<R8>) {}


pub trait CallInputs: Clone + Debug {}

impl<R: Computable, O: Operation<R>> CallInputs for (OperationWrapper<R, O>,) {}
impl<R: Computable, O: Operation<R>, R2: Computable, O2: Operation<R2>> CallInputs for (OperationWrapper<R, O>, OperationWrapper<R2, O2>) {}
impl<R: Computable, O: Operation<R>, R2: Computable, O2: Operation<R2>, R3: Computable, O3: Operation<R3>> CallInputs for (OperationWrapper<R, O>, OperationWrapper<R2, O2>, OperationWrapper<R3, O3>) {}
impl<R: Computable, O: Operation<R>, R2: Computable, O2: Operation<R2>, R3: Computable, O3: Operation<R3>, R4: Computable, O4: Operation<R4>> CallInputs for (OperationWrapper<R, O>, OperationWrapper<R2, O2>, OperationWrapper<R3, O3>,OperationWrapper<R4, O4>) {}
impl<R: Computable, O: Operation<R>, R2: Computable, O2: Operation<R2>, R3: Computable, O3: Operation<R3>, R4: Computable, O4: Operation<R4>, R5: Computable, O5: Operation<R5>> CallInputs for (OperationWrapper<R, O>, OperationWrapper<R2, O2>, OperationWrapper<R3, O3>,OperationWrapper<R4, O4>, OperationWrapper<R5, O5>) {}
impl<R: Computable, O: Operation<R>, R2: Computable, O2: Operation<R2>, R3: Computable, O3: Operation<R3>, R4: Computable, O4: Operation<R4>, R5: Computable, O5: Operation<R5>, R6: Computable, O6: Operation<R6>> CallInputs for (OperationWrapper<R, O>, OperationWrapper<R2, O2>, OperationWrapper<R3, O3>,OperationWrapper<R4, O4>, OperationWrapper<R5, O5>, OperationWrapper<R6, O6>) {}
impl<R: Computable, O: Operation<R>, R2: Computable, O2: Operation<R2>, R3: Computable, O3: Operation<R3>, R4: Computable, O4: Operation<R4>, R5: Computable, O5: Operation<R5>, R6: Computable, O6: Operation<R6>, R7: Computable, O7: Operation<R7>> CallInputs for (OperationWrapper<R, O>, OperationWrapper<R2, O2>, OperationWrapper<R3, O3>,OperationWrapper<R4, O4>, OperationWrapper<R5, O5>, OperationWrapper<R6, O6>, OperationWrapper<R7, O7>) {}
impl<R: Computable, O: Operation<R>, R2: Computable, O2: Operation<R2>, R3: Computable, O3: Operation<R3>, R4: Computable, O4: Operation<R4>, R5: Computable, O5: Operation<R5>, R6: Computable, O6: Operation<R6>, R7: Computable, O7: Operation<R7>, R8: Computable, O8: Operation<R8>> CallInputs for (OperationWrapper<R, O>, OperationWrapper<R2, O2>, OperationWrapper<R3, O3>,OperationWrapper<R4, O4>, OperationWrapper<R5, O5>, OperationWrapper<R6, O6>, OperationWrapper<R7, O7>, OperationWrapper<R8, O8>) {}


pub trait CallMatchFunctionInputs<F: FunctionInputs>: Clone {}
impl<R: Computable, O: Operation<R>> CallMatchFunctionInputs<(Variable<R>,)> for (OperationWrapper<R, O>,) {}
impl<R: Computable, O: Operation<R>, R2: Computable, O2: Operation<R2>> CallMatchFunctionInputs<(Variable<R>, Variable<R2>)> for (OperationWrapper<R, O>, OperationWrapper<R2, O2>) {}
impl<R: Computable, O: Operation<R>, R2: Computable, O2: Operation<R2>, R3: Computable, O3: Operation<R3>> CallMatchFunctionInputs<(Variable<R>, Variable<R2>, Variable<R3>)> for (OperationWrapper<R, O>, OperationWrapper<R2, O2>, OperationWrapper<R3, O3>) {}
impl<R: Computable, O: Operation<R>, R2: Computable, O2: Operation<R2>, R3: Computable, O3: Operation<R3>, R4: Computable, O4: Operation<R4>> CallMatchFunctionInputs<(Variable<R>, Variable<R2>, Variable<R3>, Variable<R4>)> for (OperationWrapper<R, O>, OperationWrapper<R2, O2>, OperationWrapper<R3, O3>, OperationWrapper<R4, O4>) {}
impl<R: Computable, O: Operation<R>, R2: Computable, O2: Operation<R2>, R3: Computable, O3: Operation<R3>, R4: Computable, O4: Operation<R4>, R5: Computable, O5: Operation<R5>> CallMatchFunctionInputs<(Variable<R>, Variable<R2>, Variable<R3>, Variable<R4>, Variable<R5>)> for (OperationWrapper<R, O>, OperationWrapper<R2, O2>, OperationWrapper<R3, O3>,OperationWrapper<R4, O4>, OperationWrapper<R5, O5>) {}
impl<R: Computable, O: Operation<R>, R2: Computable, O2: Operation<R2>, R3: Computable, O3: Operation<R3>, R4: Computable, O4: Operation<R4>, R5: Computable, O5: Operation<R5>, R6: Computable, O6: Operation<R6>> CallMatchFunctionInputs<(Variable<R>, Variable<R2>, Variable<R3>, Variable<R4>, Variable<R5>, Variable<R6>)> for (OperationWrapper<R, O>, OperationWrapper<R2, O2>, OperationWrapper<R3, O3>,OperationWrapper<R4, O4>, OperationWrapper<R5, O5>, OperationWrapper<R6, O6>) {}
impl<R: Computable, O: Operation<R>, R2: Computable, O2: Operation<R2>, R3: Computable, O3: Operation<R3>, R4: Computable, O4: Operation<R4>, R5: Computable, O5: Operation<R5>, R6: Computable, O6: Operation<R6>, R7: Computable, O7: Operation<R7>> CallMatchFunctionInputs<(Variable<R>, Variable<R2>, Variable<R3>, Variable<R4>, Variable<R5>, Variable<R6>, Variable<R7>)> for (OperationWrapper<R, O>, OperationWrapper<R2, O2>, OperationWrapper<R3, O3>,OperationWrapper<R4, O4>, OperationWrapper<R5, O5>, OperationWrapper<R6, O6>, OperationWrapper<R7, O7>) {}
impl<R: Computable, O: Operation<R>, R2: Computable, O2: Operation<R2>, R3: Computable, O3: Operation<R3>, R4: Computable, O4: Operation<R4>, R5: Computable, O5: Operation<R5>, R6: Computable, O6: Operation<R6>, R7: Computable, O7: Operation<R7>, R8: Computable, O8: Operation<R8>> CallMatchFunctionInputs<(Variable<R>, Variable<R2>, Variable<R3>, Variable<R4>, Variable<R5>, Variable<R6>, Variable<R7>, Variable<R8>)> for (OperationWrapper<R, O>, OperationWrapper<R2, O2>, OperationWrapper<R3, O3>,OperationWrapper<R4, O4>, OperationWrapper<R5, O5>, OperationWrapper<R6, O6>, OperationWrapper<R7, O7>, OperationWrapper<R8, O8>) {}

pub trait DiffableFunctionInputs<R: Computable, F: FunctionInputs>: CallInputs + CallMatchFunctionInputs<F> {
    type Diff<OF: Differentiable<R>>: Operation<R>;

    fn contains_var<RV: Clone>(&self, var: Variable<RV>) -> bool;
    fn map_variable<RV: Clone>(&self, var: Variable<RV>, f_inputs: F) -> Option<Variable<RV>>;
    fn auto_diff_for<RV: Clone, O: Differentiable<R>>(&self, call: Call<R, F, Self, O>, var: Variable<RV>, og: Variable<RV>, var_trace: &mut HashMap<String, Vec<String>>) -> Self::Diff<O>;
}

impl<R: Calculatable, O: Differentiable<R>> DiffableFunctionInputs<R, (Variable<R>,)> for (OperationWrapper<R, O>,) {
    type Diff<OF: Differentiable<R>> = OperationWrapper<R, Multiply<R, Call<R, (Variable<R>,), Self, OF::Diff>, OperationWrapper<R, O::Diff>>>;

    fn contains_var<RV: Clone>(&self, var: Variable<RV>) -> bool {
        self.0.contains_var(var)
    }

    fn map_variable<RV: Clone>(&self, var: Variable<RV>, f_inputs: (Variable<R>,)) -> Option<Variable<RV>> {
        if self.0.contains_var(var.clone()) {
            let mut result = var;
            result.reference = f_inputs.0.reference.clone();
            Some(result)
        } else {
            None
        }
    }

    fn auto_diff_for<RV: Clone, OF: Differentiable<R>>(&self, call: Call<R, (Variable<R>,), Self, OF>, var: Variable<RV>, og: Variable<RV>, var_trace: &mut HashMap<String, Vec<String>>) -> Self::Diff<OF> {
        multiply(Call { inputs: self.clone(), function: call.function.auto_diff_for(var.clone(), var_trace) }, OperationWrapper(self.0.0.auto_diff_for(og, var_trace), PhantomData)) //(OperationWrapper(self.0.0.auto_diff_for(var, var_trace), PhantomData),)
    }
}

impl<R: Calculatable, O: Differentiable<R>, O2: Differentiable<R>> DiffableFunctionInputs<R, (Variable<R>, Variable<R>)> for (OperationWrapper<R, O>, OperationWrapper<R, O2>) {
    type Diff<OF: Differentiable<R>> = OperationWrapper<R, Multiply<R, Call<R, (Variable<R>, Variable<R>), Self, OF::Diff>, OperationWrapper<R, Multiply<R, OperationWrapper<R, O::Diff>, OperationWrapper<R, O2::Diff>>>>>;

    fn contains_var<RV: Clone>(&self, var: Variable<RV>) -> bool {
        self.0.contains_var(var.clone()) || self.1.contains_var(var)
    }

    fn map_variable<RV: Clone>(&self, var: Variable<RV>, f_inputs: (Variable<R>, Variable<R>)) -> Option<Variable<RV>> {
        if self.0.contains_var(var.clone()) {
            let mut result = var;
            result.reference = f_inputs.0.reference.clone();
            Some(result)
        } else if self.1.contains_var(var.clone()) {
            let mut result = var;
            result.reference = f_inputs.1.reference.clone();
            Some(result)
        } else {
            None
        }
    }

    fn auto_diff_for<RV: Clone, OF: Differentiable<R>>(&self, call: Call<R, (Variable<R>, Variable<R>), Self, OF>, var: Variable<RV>, og: Variable<RV>, var_trace: &mut HashMap<String, Vec<String>>) -> Self::Diff<OF> {
        multiply(Call { inputs: self.clone(), function: call.function.auto_diff_for(var.clone(), var_trace) }, multiply(OperationWrapper(self.0.0.auto_diff_for(og.clone(), var_trace), PhantomData), OperationWrapper(self.1.0.auto_diff_for(og, var_trace), PhantomData))) 
    }

}


pub trait Generalizable: Clone + Send + Sync {
    fn get_memory_layout() -> MemoryLayoutDescriptor;

    fn get_parallelization_info(&self) -> ParallelizationDescriptor;
}



pub trait FromMut<T> {
    type Result<'a>: Clone + Send + Sync where T: 'a;
    fn from_mut<'a>(value: &'a mut T) -> Self::Result<'_>;
}

impl<T: Clone + Send + Sync> FromMut<T> for T {
    type Result<'a> = T where T: 'a;

    fn from_mut<'a>(value: &'a mut T) -> Self::Result<'_> {
        value.clone()
    }
}

pub trait MemoryMapable<S: Storage>: Generalizable + 'static {
    type Mapped<'a>: FromMut<S::MappedType<Self>> + Clone;

    fn get_struct_map(&self) -> HashMap<String, String>;

    fn to_memory_bytes(&self) -> Vec<u8>;
    fn from_memory_bytes(bytes: Vec<u8>) -> Self;
    fn into_processor_mapped(self) -> S::MappedType<Self>;
}

pub trait Calculatable: Computable + ::core::ops::Add<Self, Output = Self> + ::core::ops::Sub<Self, Output = Self> + ::core::ops::Mul<Self, Output = Self>  + std::cmp::PartialOrd + ::core::ops::Div<Self, Output = Self> {}

impl<C: Computable> Calculatable for C where C: ::core::ops::Add<C, Output = C> + ::core::ops::Sub<C, Output = C> + ::core::ops::Mul<C, Output = C> + std::cmp::PartialOrd + ::core::ops::Div<C, Output = C> {

}
