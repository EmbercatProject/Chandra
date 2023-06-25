use std::{any::Any, rc::Rc};

use crate::processor::cpu::{CPUStorage};

use crate::core::{type_traits::{MemoryMapable}, allocated::{ExecutableBindings, Binding}, Buildable};

pub mod cpu;

pub trait ProcessorInformation where Self: Sized {
    type Storage: Storage;
    type Executable<B: Buildable<Self>>;
    type Compiler;
}

pub trait Processor where Self: Sized + ProcessorInformation {
    fn build<B: Buildable<Self>>(&mut self, buildable: B) -> Self::Executable<B>;
    fn alloc<T: MemoryMapable<Self::Storage>>(&mut self, val: T) -> Binding<Self::Storage, T>;

    fn dealloc<T: MemoryMapable<Self::Storage>>(&mut self, val: Binding<Self::Storage, T>) -> T;
    fn copy_to_cpu<T: MemoryMapable<Self::Storage>>(&mut self, val: &Binding<Self::Storage, T>) -> T;

    fn dispatch<'a, B: Buildable<Self>>(&'a mut self, executable: &'a mut Self::Executable<B>, x: u32, y: u32, z: u32)
        where <B::Binding as ExecutableBindings<Self::Storage>>::O: 'static, <B::CPUBinding as ExecutableBindings<CPUStorage>>::O: 'static;
}

pub trait Executable<S: Storage, B: ExecutableBindings<S>> {
    fn get_bindings(&mut self) -> &mut B;
    fn get_bindings_ref(&self) -> &B;
}

pub trait ReferencCounter: Clone {
    type Inner;

    fn is_last_reference(&self) -> bool;
    fn get_ref(&self) -> &Self::Inner;
}

impl<T> ReferencCounter for Rc<T> {
    type Inner = T;

    fn is_last_reference(&self) -> bool {
        Rc::<T>::strong_count(self) == 1
    }

    fn get_ref(&self) -> &Self::Inner {
        self.as_ref()
    }
}

pub trait Storage: Clone {
    type Key: ReferencCounter;
    type Data<T>: ReferencCounter;
    type MappedType<T: Any + Clone +  Send + Sync>: Any + Clone + Send + Sync;

    fn remove<V: Any + Clone + Send + Sync>(&mut self, key: &Self::Key) -> Option<Self::MappedType<V>>;
}