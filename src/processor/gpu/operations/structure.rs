use std::collections::HashMap;

use crate::core::{types::{Void}, operations::{function::Function, call::Call, instruction_list::InstructionList, noop::Noop, scope::Scope, var::Variable}, type_traits::{FunctionInputs, CallMatchFunctionInputs, CallInputs}, operation::OperationWrapper};

use super::{GPUOperation, GPUValue, GPUComputable};



impl<R: GPUValue, INPUTS: FunctionInputs + GPUFunctionInputs, A: GPUOperation<R>> GPUOperation<R> for Function<R, INPUTS, A> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let inputs = self.inputs.build();
        let block = self.scope.build(functions);

        format!("fn {}({}) {} {}", self.name, inputs, R::get_return_decl(), block)
    }
}

impl<R: GPUValue, INPUTS: FunctionInputs + GPUFunctionInputs, CALLINPUTS: GPUCallInputs + CallMatchFunctionInputs<INPUTS>, A: GPUOperation<R>> GPUOperation<R> for Call<R, INPUTS, CALLINPUTS, A> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        if !functions.contains_key(&self.function.name) {
            let func = self.function.build(functions);
            functions.insert(self.function.name.clone(), func);
        }
        let inputs = self.inputs.build(functions);

        format!("{}({})", self.function.name, inputs)
    }
}


impl<R: GPUValue, R2: GPUValue, A: GPUOperation<R>, B: GPUOperation<R2>> GPUOperation<R> for InstructionList<R, R2, A, B> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        if let Some(prev) = &self.previous {
            let p = prev.build(functions);
            let t = self.this.build(functions);
            format!("{} \n{}", p, t)
        } else {
            let t = self.this.build(functions);
            format!("{}", t)
        }
    }
}

impl GPUOperation<Void> for Noop {
    fn build(&self, _functions: &mut HashMap<String, String>) -> String {
        String::new()
    }
}

impl<R: GPUValue, R2: GPUValue, A: GPUOperation<R>, B: GPUOperation<R2>> GPUOperation<R> for Scope<R, R, R2, A, B> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        format!("{{
    {}
}}", self.instruction.build(functions))
    }
}


pub trait GPUCallInputs: CallInputs {
    fn build(&self, functions: &mut HashMap<String, String>) -> String;
}

impl<R0: GPUComputable, O0: GPUOperation<R0>> GPUCallInputs for (OperationWrapper<R0, O0>,) {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let i0 = self.0.build(functions);
        format!("{}", i0)
    }
}
impl<R: GPUComputable, O: GPUOperation<R>, R2: GPUComputable, O2: GPUOperation<R2>> GPUCallInputs for (OperationWrapper<R, O>, OperationWrapper<R2, O2>) {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let i0 = self.0.build(functions);
        let i1 = self.1.build(functions);
        format!("{}, {}", i0, i1)
    }
}
impl<R: GPUComputable, O: GPUOperation<R>, R2: GPUComputable, O2: GPUOperation<R2>, R3: GPUComputable, O3: GPUOperation<R3>> GPUCallInputs for (OperationWrapper<R, O>, OperationWrapper<R2, O2>, OperationWrapper<R3, O3>) {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let i0 = self.0.build(functions);
        let i1 = self.1.build(functions);
        let i2 = self.2.build(functions);
        format!("{}, {}, {}", i0, i1, i2)
    }
}
impl<R: GPUComputable, O: GPUOperation<R>, R2: GPUComputable, O2: GPUOperation<R2>, R3: GPUComputable, O3: GPUOperation<R3>, R4: GPUComputable, O4: GPUOperation<R4>> GPUCallInputs for (OperationWrapper<R, O>, OperationWrapper<R2, O2>, OperationWrapper<R3, O3>,OperationWrapper<R4, O4>) {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let i0 = self.0.build(functions);
        let i1 = self.1.build(functions);
        let i2 = self.2.build(functions);
        let i3 = self.3.build(functions);
        format!("{}, {}, {}, {}", i0, i1, i2, i3)
    }
}
impl<R: GPUComputable, O: GPUOperation<R>, R2: GPUComputable, O2: GPUOperation<R2>, R3: GPUComputable, O3: GPUOperation<R3>, R4: GPUComputable, O4: GPUOperation<R4>, R5: GPUComputable, O5: GPUOperation<R5>> GPUCallInputs for (OperationWrapper<R, O>, OperationWrapper<R2, O2>, OperationWrapper<R3, O3>,OperationWrapper<R4, O4>, OperationWrapper<R5, O5>) {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let i0 = self.0.build(functions);
        let i1 = self.1.build(functions);
        let i2 = self.2.build(functions);
        let i3 = self.3.build(functions);
        let i4 = self.4.build(functions);
        format!("{}, {}, {}, {}, {}", i0, i1, i2, i3, i4)
    }
}
impl<R: GPUComputable, O: GPUOperation<R>, R2: GPUComputable, O2: GPUOperation<R2>, R3: GPUComputable, O3: GPUOperation<R3>, R4: GPUComputable, O4: GPUOperation<R4>, R5: GPUComputable, O5: GPUOperation<R5>, R6: GPUComputable, O6: GPUOperation<R6>> GPUCallInputs for (OperationWrapper<R, O>, OperationWrapper<R2, O2>, OperationWrapper<R3, O3>,OperationWrapper<R4, O4>, OperationWrapper<R5, O5>, OperationWrapper<R6, O6>) {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let i0 = self.0.build(functions);
        let i1 = self.1.build(functions);
        let i2 = self.2.build(functions);
        let i3 = self.3.build(functions);
        let i4 = self.4.build(functions);
        let i5 = self.5.build(functions);
        format!("{}, {}, {}, {}, {}, {}", i0, i1, i2, i3, i4, i5)
    }
}
impl<R: GPUComputable, O: GPUOperation<R>, R2: GPUComputable, O2: GPUOperation<R2>, R3: GPUComputable, O3: GPUOperation<R3>, R4: GPUComputable, O4: GPUOperation<R4>, R5: GPUComputable, O5: GPUOperation<R5>, R6: GPUComputable, O6: GPUOperation<R6>, R7: GPUComputable, O7: GPUOperation<R7>> GPUCallInputs for (OperationWrapper<R, O>, OperationWrapper<R2, O2>, OperationWrapper<R3, O3>,OperationWrapper<R4, O4>, OperationWrapper<R5, O5>, OperationWrapper<R6, O6>, OperationWrapper<R7, O7>) {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let i0 = self.0.build(functions);
        let i1 = self.1.build(functions);
        let i2 = self.2.build(functions);
        let i3 = self.3.build(functions);
        let i4 = self.4.build(functions);
        let i5 = self.5.build(functions);
        let i6 = self.6.build(functions);
        format!("{}, {}, {}, {}, {}, {}, {}", i0, i1, i2, i3, i4, i5, i6)
    }
}
impl<R: GPUComputable, O: GPUOperation<R>, R2: GPUComputable, O2: GPUOperation<R2>, R3: GPUComputable, O3: GPUOperation<R3>, R4: GPUComputable, O4: GPUOperation<R4>, R5: GPUComputable, O5: GPUOperation<R5>, R6: GPUComputable, O6: GPUOperation<R6>, R7: GPUComputable, O7: GPUOperation<R7>, R8: GPUComputable, O8: GPUOperation<R8>> GPUCallInputs for (OperationWrapper<R, O>, OperationWrapper<R2, O2>, OperationWrapper<R3, O3>,OperationWrapper<R4, O4>, OperationWrapper<R5, O5>, OperationWrapper<R6, O6>, OperationWrapper<R7, O7>, OperationWrapper<R8, O8>) {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let i0 = self.0.build(functions);
        let i1 = self.1.build(functions);
        let i2 = self.2.build(functions);
        let i3 = self.3.build(functions);
        let i4 = self.4.build(functions);
        let i5 = self.5.build(functions);
        let i6 = self.6.build(functions);
        let i7 = self.7.build(functions);
        format!("{}, {}, {}, {}, {}, {}, {}, {}", i0, i1, i2, i3, i4, i5, i6, i7)
    }
}

pub trait GPUFunctionInputs: FunctionInputs {
    fn build(&self) -> String;
}

impl<R: GPUComputable> GPUFunctionInputs for (Variable<R>,) {
    fn build(&self) -> String {
        let i0 = &self.0.reference;
        let t0 = R::get_type_info();
        format!("{}: {}", i0, t0)
    }
}
impl<R: GPUComputable, R2: GPUComputable> GPUFunctionInputs for (Variable<R>, Variable<R2>) {
    fn build(&self) -> String {
        let i0 = &self.0.reference;
        let t0 = R::get_type_info();

        let i1 = &self.1.reference;
        let t1 = R2::get_type_info();
        format!("{}: {}, {}: {}", i0, t0, i1, t1)
    }
}
impl<R: GPUComputable, R2: GPUComputable, R3: GPUComputable> GPUFunctionInputs for (Variable<R>, Variable<R2>, Variable<R3>) {
    fn build(&self) -> String {
        let i0 = &self.0.reference;
        let t0 = R::get_type_info();

        let i1 = &self.1.reference;
        let t1 = R2::get_type_info();

        let i2 = &self.2.reference;
        let t2 = R3::get_type_info();
        format!("{}: {}, {}: {}, {}: {}", i0, t0, i1, t1, i2, t2)
    }
}
impl<R: GPUComputable, R2: GPUComputable, R3: GPUComputable, R4: GPUComputable> GPUFunctionInputs for (Variable<R>, Variable<R2>, Variable<R3>, Variable<R4>) {
    fn build(&self) -> String {
        let i0 = &self.0.reference;
        let t0 = R::get_type_info();

        let i1 = &self.1.reference;
        let t1 = R2::get_type_info();

        let i2 = &self.2.reference;
        let t2 = R3::get_type_info();

        let i3 = &self.3.reference;
        let t3 = R4::get_type_info();
        format!("{}: {}, {}: {}, {}: {}, {}: {}", i0, t0, i1, t1, i2, t2, i3, t3)
    }
}
impl<R: GPUComputable, R2: GPUComputable, R3: GPUComputable, R4: GPUComputable, R5: GPUComputable> GPUFunctionInputs for (Variable<R>, Variable<R2>, Variable<R3>, Variable<R4>, Variable<R5>) {
    fn build(&self) -> String {
        let i0 = &self.0.reference;
        let t0 = R::get_type_info();

        let i1 = &self.1.reference;
        let t1 = R2::get_type_info();

        let i2 = &self.2.reference;
        let t2 = R3::get_type_info();

        let i3 = &self.3.reference;
        let t3 = R4::get_type_info();

        let i4 = &self.4.reference;
        let t4 = R5::get_type_info();
        format!("{}: {}, {}: {}, {}: {}, {}: {}, {}: {}", i0, t0, i1, t1, i2, t2, i3, t3, i4, t4)
    }
}
impl<R: GPUComputable, R2: GPUComputable, R3: GPUComputable, R4: GPUComputable, R5: GPUComputable, R6: GPUComputable> GPUFunctionInputs for (Variable<R>, Variable<R2>, Variable<R3>, Variable<R4>, Variable<R5>, Variable<R6>) {
    fn build(&self) -> String {
        let i0 = &self.0.reference;
        let t0 = R::get_type_info();

        let i1 = &self.1.reference;
        let t1 = R2::get_type_info();

        let i2 = &self.2.reference;
        let t2 = R3::get_type_info();

        let i3 = &self.3.reference;
        let t3 = R4::get_type_info();

        let i4 = &self.4.reference;
        let t4 = R5::get_type_info();

        let i5 = &self.5.reference;
        let t5 = R6::get_type_info();
        format!("{}: {}, {}: {}, {}: {}, {}: {}, {}: {}, {}: {}", i0, t0, i1, t1, i2, t2, i3, t3, i4, t4, i5, t5)
    }
}
impl<R: GPUComputable, R2: GPUComputable, R3: GPUComputable, R4: GPUComputable, R5: GPUComputable, R6: GPUComputable, R7: GPUComputable> GPUFunctionInputs for (Variable<R>, Variable<R2>, Variable<R3>, Variable<R4>, Variable<R5>, Variable<R6>, Variable<R7>) {
    fn build(&self) -> String {
        let i0 = &self.0.reference;
        let t0 = R::get_type_info();

        let i1 = &self.1.reference;
        let t1 = R2::get_type_info();

        let i2 = &self.2.reference;
        let t2 = R3::get_type_info();

        let i3 = &self.3.reference;
        let t3 = R4::get_type_info();

        let i4 = &self.4.reference;
        let t4 = R5::get_type_info();

        let i5 = &self.5.reference;
        let t5 = R6::get_type_info();

        let i6 = &self.6.reference;
        let t6 = R7::get_type_info();
        format!("{}: {}, {}: {}, {}: {}, {}: {}, {}: {}, {}: {}, {}: {}", i0, t0, i1, t1, i2, t2, i3, t3, i4, t4, i5, t5, i6, t6)
    }
}
impl<R: GPUComputable, R2: GPUComputable, R3: GPUComputable, R4: GPUComputable, R5: GPUComputable, R6: GPUComputable, R7: GPUComputable, R8: GPUComputable> GPUFunctionInputs for (Variable<R>, Variable<R2>, Variable<R3>, Variable<R4>, Variable<R5>, Variable<R6>, Variable<R7>, Variable<R8>) {
    fn build(&self) -> String {
        let i0 = &self.0.reference;
        let t0 = R::get_type_info();

        let i1 = &self.1.reference;
        let t1 = R2::get_type_info();

        let i2 = &self.2.reference;
        let t2 = R3::get_type_info();

        let i3 = &self.3.reference;
        let t3 = R4::get_type_info();

        let i4 = &self.4.reference;
        let t4 = R5::get_type_info();

        let i5 = &self.5.reference;
        let t5 = R6::get_type_info();

        let i6 = &self.6.reference;
        let t6 = R7::get_type_info();

        let i7 = &self.7.reference;
        let t7 = R8::get_type_info();
        format!("{}: {}, {}: {}, {}: {}, {}: {}, {}: {}, {}: {}, {}: {}, {}: {}", i0, t0, i1, t1, i2, t2, i3, t3, i4, t4, i5, t5, i6, t6, i7, t7)
    }
}