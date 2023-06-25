use std::{any::Any, fmt::Debug};

use crate::{core::{any_map::AnyMap, allocated::{ExecutableBindings, ToRawInputs}, types::{Pos}, type_traits::{MemoryMapable, FromMut}}, processor::cpu::CPUStorage};

use super::{Storage};


pub trait CPUFunction<B: ExecutableBindings<CPUStorage>>: Send + Sync + Clone + Debug {
    fn call_cpu<'a, 'b>(&self, pos: &Pos, inputs: &<<B as ExecutableBindings<CPUStorage>>::I as ToRawInputs>::RawDerefed<'a>, output: &mut <<<B as ExecutableBindings<CPUStorage>>::O as MemoryMapable<CPUStorage>>::Mapped<'b> as FromMut<<CPUStorage as Storage>::MappedType<<B as ExecutableBindings<CPUStorage>>::O>>>::Result<'b>);
}

pub struct DifferentiatedCPUContext(AnyMap<String>, bool);

impl DifferentiatedCPUContext {
    pub fn get<K: Any>(&self, reference: &str) -> &K {
        self.0.get(reference.to_string()).unwrap()
    }

    pub fn get_mut<K: Any>(&mut self, reference: &str) -> &mut K {
        self.0.get_mut(reference.to_string()).unwrap()
    }

    pub fn set<K: Any>(&mut self, reference: &str, value: K) -> Option<K> {
        self.0.insert(reference.to_string(), value)
    }

    pub fn remove<K: Any>(&mut self, reference: &str, _value: K) {
        self.0.remove::<K>(reference.to_string());
    }

    pub fn is_in_return_state(&self) -> bool {
        self.1
    }

    pub fn set_return_state(&mut self, state: bool) {
        self.1 = state;
    }
}