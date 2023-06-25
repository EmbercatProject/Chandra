use std::marker::PhantomData;

use chandra_kernel::ChandraFunction;

use crate::core::{type_traits::Calculatable, operation::{Operation, OperationWrapper}, types::{Value, Void}, operations::{get::{Get, self}, noop::Noop, function::{Function, self}, var::Variable, scope::Scope, instruction_list::InstructionList, returns::returns}};


#[derive(Clone, Debug)]
pub struct Log<R: Calculatable, V: Operation<R>> {
    value: V,
    _0: PhantomData<R>
}

impl<R: Calculatable, V: Operation<R>> Operation<R> for Log<R, V> {
    fn evaluate(&self, _context: &mut crate::core::processor::cpu::DifferentiatedCPUContext) -> R {
        todo!()
    }
}

//pub struct TestLN<R: Calculatable> {
//    scope: Function<R, (Variable<R>,), Log<R, OperationWrapper<R, Get<R>>>, InstructionList<Void, Void, Noop, Noop>>
//}
//
//impl<R: Calculatable> TestLN<R> {
//    pub fn new(inputs: (Variable<R>, )) -> Self {
//        Self { 
//            scope: function::function("TestLN", inputs, |v| {
//                Scope::new().returns(returns(Log::<R, OperationWrapper<R, Get<R>>> { value: get::get(&v.0), _0: PhantomData }))
//            })
//        }
//    }
//}


//#[ChandraFunction]
//pub fn ln<T: Calculatable>(val: &T) -> T {
//    Log::<T, OperationWrapper<T, Get<T>>> { value: val, _0: PhantomData }
//}
