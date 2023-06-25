use std::collections::HashMap;

use crate::core::{type_traits::{IndexAble, GetAndSetable}, operations::{index::Index, assign::Assign, get::Get, set::Set, var::Variable}, types::Void};

use super::{GPUOperation, GPUComputable};

impl<R: GPUComputable, T: IndexAble, O: GPUOperation<u32>> GPUOperation<R> for Index<R, T, O> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        format!("{}[{}]", self.tensor.get_reference(), self.index.build(functions))
    }
}


impl<R: GPUComputable, Value: GPUOperation<R>> GPUOperation<Void> for Assign<R, Value> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        //var a: i32 = 20;
        format!("var {}: {} = {};", &self.variable.reference, R::get_type_info(), self.assign.build(functions))
    }
}

impl<R: GPUComputable> GPUOperation<R> for Get<R> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        self.getable.build(functions)
    }
}


impl<R: GPUComputable, G: GetAndSetable<R> + GPUOperation<R>, O: GPUOperation<R>> GPUOperation<Void> for Set<R, G, O> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let set = self.getable.build(functions);
        let assign = self.assign.build(functions);
        format!("{} = {};", set, assign)
    }
}


impl<R: GPUComputable> GPUOperation<R> for Variable<R> {
    fn build(&self, _functions: &mut HashMap<String, String>) -> String {
        self.reference.clone()
    }
}