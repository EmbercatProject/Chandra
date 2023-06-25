use std::{rc::Rc, cell::RefCell, collections::HashMap};

use crate::core::{processor::{ProcessorInformation, Storage, Processor, Executable}, types::Void, operation::Compilable, operations::scope::Scope, allocated::ExecutableBindings, Buildable};

use super::operations::GPUOperation;

pub struct GPUProcessor {

}

impl GPUProcessor {
    pub fn new() -> Self {
        GPUProcessor {  }
    }
}

#[derive(Clone)]
pub struct GPUStorage {

}

pub struct GPUCompiler(String);

impl Storage for GPUStorage {
    type Key = Rc<usize>;

    type Data<T> = Rc<RefCell<T>>;

    type MappedType<T: std::any::Any + Clone +  Send + Sync> = usize;

    fn remove<V: std::any::Any + Clone + Send + Sync>(&mut self, key: &Self::Key) -> Option<Self::MappedType<V>> {
        todo!()
    }
}

pub struct GPUExecutable {
    prog: String
}

impl<B: ExecutableBindings<GPUStorage>> Executable<GPUStorage, B> for GPUExecutable {
    fn get_bindings(&mut self) -> &mut B {
        todo!()
    }
    fn get_bindings_ref(&self) -> & B {
        todo!()
    }
}

impl ProcessorInformation for GPUProcessor {
    type Storage = GPUStorage;

    type Executable<B: crate::core::Buildable<Self>> = GPUExecutable;

    type Compiler = GPUCompiler;
}

impl<A: GPUOperation<Void>, B: GPUOperation<Void>> Compilable<Void, GPUCompiler> for Scope<Void, Void, Void, A, B> {
    fn build(&self, compiler: &mut GPUCompiler) {
        let mut functions = HashMap::new();
        let main = GPUOperation::<Void>::build(self, &mut functions);

        let funcs = functions.into_iter()
            .fold(String::new(), |before, (_k, v)| format!("{} \n {}", before, v));
        compiler.0 = format!("{} \n fn main() {}", funcs, main);
    }
}

impl Processor for GPUProcessor {
    fn build<B: crate::core::Buildable<Self>>(&mut self, buildable: B) -> Self::Executable<B> {
        let mut compiler = GPUCompiler(String::new());
        buildable.get_main_tree().build(&mut compiler);
        println!("{}", compiler.0);
        GPUExecutable {
            prog: compiler.0.clone(),
        }
    }

    fn alloc<T: crate::core::type_traits::MemoryMapable<Self::Storage>>(&mut self, val: T) -> crate::core::allocated::Binding<Self::Storage, T> {
        todo!()
    }

    fn dealloc<T: crate::core::type_traits::MemoryMapable<Self::Storage>>(&mut self, val: crate::core::allocated::Binding<Self::Storage, T>) -> T {
        todo!()
    }

    fn copy_to_cpu<T: crate::core::type_traits::MemoryMapable<Self::Storage>>(&mut self, val: &crate::core::allocated::Binding<Self::Storage, T>) -> T {
        todo!()
    }

    fn dispatch<'a, B: crate::core::Buildable<Self>>(&'a mut self, executable: &'a mut Self::Executable<B>, x: u32, y: u32, z: u32)
        where <B::Binding as crate::core::allocated::ExecutableBindings<Self::Storage>>::O: 'static, <B::CPUBinding as crate::core::allocated::ExecutableBindings<crate::processor::cpu::CPUStorage>>::O: 'static {
        todo!()
    }
}