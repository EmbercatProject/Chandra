use std::{rc::{Rc}, marker::PhantomData, cell::{RefMut, Ref, RefCell}, collections::HashMap};

use super::{type_traits::MemoryMapable, processor::{Storage, ReferencCounter}};
use crate::processor::cpu::{CPUStorage};

pub trait ProgrammInputs: ToRawInputs {    
    fn to_raw<'a> (&'a self) -> Self::Raw<'a>;
    fn deref_raw<'a, 'b> (&self, raw: &'a Self::Raw<'b>) -> Self::RawDerefed<'a>;
}


#[derive(Clone)]
pub enum ParallelizationDescriptor {
    Struct(StructParallelizationDescriptor),
    Data(Parallelizable)
}

#[derive(Clone)]
pub enum Parallelizable {
    X(u64),
    XY(u64, u64),
    FULL,
    Sync
}

#[derive(Clone)]
pub struct StructParallelizationDescriptor {
    pub this: Parallelizable,
    pub fields: HashMap<String, (u8, MemoryLayoutDescriptor, ParallelizationDescriptor)>
}

#[derive(Clone)]
pub enum MemoryLayoutDescriptor {
    Array {item_typ: Box<MemoryLayoutDescriptor>, item_length: usize},
    Float(u8),
    Integer(u8),
    UInteger(u8),
    Struct(HashMap<String, (u8, MemoryLayoutDescriptor)>),
    Vector {item_typ: Box<MemoryLayoutDescriptor> },

    Empty,

    Custom {custom_typ: String, byte_size: usize, extra: Vec<u8>}
}

impl MemoryLayoutDescriptor {
    pub fn get_bytes_before(&self, entry: &str) -> u64 {
        if let Self::Struct(map) = self {
            let (pos, _) = map.get(entry).expect("Existence");

            let mut bytes = 0;

            for (_n, (p, l)) in map.iter() {
                if p < pos {
                    bytes += l.get_byte_size();
                }
            }

            bytes
        } else {
            panic!("Bad use")
        }
    }

    pub fn get_byte_size(&self) -> u64 {
        match self {
            MemoryLayoutDescriptor::Float(x) => *x as u64,
            MemoryLayoutDescriptor::Integer(x) => *x as u64,
            MemoryLayoutDescriptor::UInteger(x) => *x as u64,
            MemoryLayoutDescriptor::Struct(m) => {
                m.iter()
                    .map(|(_, l)| l.1.get_byte_size())
                    .sum()
            }

            _ => unimplemented!()
        }
    }
}

//pub trait ToRawInputs<S: Storage, C: RawInputs<S>> {
//}
//
//impl<S: Storage, T: MemoryMapable<S> + 'static> ToRawInputs<S, (&T,)> for (Binding<S, T>,) { }
//impl<S: Storage, T1: MemoryMapable<S> + 'static, T2: MemoryMapable<S> + 'static> ToRawInputs<S, (&T1, &T2)> for (Binding<S, T1>, Binding<S, T2>) { }
//impl<S: Storage, T1: MemoryMapable<S> + 'static, T2: MemoryMapable<S> + 'static, T3: MemoryMapable<S> + 'static> ToRawInputs<S, (&T1, &T2, &T3)> for (Binding<S, T1>, Binding<S, T2>, Binding<S, T3>) {}
//impl<S: Storage, T1: MemoryMapable<S> + 'static, T2: MemoryMapable<S> + 'static, T3: MemoryMapable<S> + 'static, T4: MemoryMapable<S> + 'static> ToRawInputs<S, (&T1, &T2, &T3, &T4)> for (Binding<S, T1>, Binding<S, T2>, Binding<S, T3>, Binding<S, T4>) {}

pub trait ToRawInputs {
    type Raw<'a>: InnerClonable;
    type RawDerefed<'a>: Send + Sync;
    type Storage: Storage;
}

pub trait InnerClonable {
    fn iclone(&self) -> Self;
}

impl<'a, T> InnerClonable for (Ref<'a, T>,) {
    fn iclone(&self) -> Self {
        (Ref::clone(&self.0),)
    }
}
impl<'a, T1, T2> InnerClonable for (Ref<'a, T1>, Ref<'a, T2>) {
    fn iclone(&self) -> Self {
       (Ref::clone(&self.0),Ref::clone(&self.1))
    }
}
impl<'a, T1, T2, T3> InnerClonable for (Ref<'a, T1>, Ref<'a, T2>, Ref<'a, T3>) {
    fn iclone(&self) -> Self {
       (Ref::clone(&self.0), Ref::clone(&self.1), Ref::clone(&self.2))
    }
}

impl<S: Storage, T: MemoryMapable<S> + 'static> ToRawInputs for (Binding<S, T>,) {
    type Raw<'a> = (Ref<'a, <S as Storage>::MappedType<T>>,);
    type RawDerefed<'a> = (&'a <S as Storage>::MappedType<T>,);
    type Storage = S;
}
impl<S: Storage, T1: MemoryMapable<S> + 'static, T2: MemoryMapable<S> + 'static> ToRawInputs for (Binding<S, T1>, Binding<S, T2>) {
    type Raw<'a> = (Ref<'a, <S as Storage>::MappedType<T1>>, Ref<'a, <S as Storage>::MappedType<T2>>);
    type RawDerefed<'a> = (&'a <S as Storage>::MappedType<T1>, &'a <S as Storage>::MappedType<T2>);
    type Storage = S;
}
impl<S: Storage, T1: MemoryMapable<S> + 'static, T2: MemoryMapable<S> + 'static, T3: MemoryMapable<S> + 'static> ToRawInputs for (Binding<S, T1>, Binding<S, T2>, Binding<S, T3>) {
    type Raw<'a> = (Ref<'a, <S as Storage>::MappedType<T1>>, Ref<'a, <S as Storage>::MappedType<T2>>, Ref<'a, <S as Storage>::MappedType<T3>>);
    type RawDerefed<'a> = (&'a <S as Storage>::MappedType<T1>, &'a <S as Storage>::MappedType<T2>, &'a <S as Storage>::MappedType<T3>);
    type Storage = S;
}
//impl<S: Storage, T1: MemoryMapable<S> + 'static, T2: MemoryMapable<S> + 'static, T3: MemoryMapable<S> + 'static> RawInputs<S> for (&T1, &T2, &T3) {}
//impl<S: Storage, T1: MemoryMapable<S> + 'static, T2: MemoryMapable<S> + 'static, T3: MemoryMapable<S> + 'static, T4: MemoryMapable<S> + 'static> RawInputs<S> for (&T1, &T2, &T3, &T4) {}

impl<S: Storage, T: MemoryMapable<S> + 'static> ProgrammInputs for (Binding<S, T>,) {
    fn to_raw<'a>(&'a self) -> Self::Raw<'a> {
        (self.0.deref(),)
    }

    fn deref_raw<'a, 'c> (&self, raw: &'a Self::Raw<'c>) -> Self::RawDerefed<'a> {
        (&*raw.0,)
    }
}
impl<S: Storage, T1: MemoryMapable<S> + 'static, T2: MemoryMapable<S> + 'static> ProgrammInputs for (Binding<S, T1>, Binding<S, T2>) {
    fn to_raw<'a>(&'a self) -> Self::Raw<'a> {
        (self.0.deref(), self.1.deref())
    }

    fn deref_raw<'a, 'c> (&self, raw: &'a Self::Raw<'c>) -> Self::RawDerefed<'a> {
        (&*raw.0, &*raw.1)
    }
}
impl<S: Storage, T1: MemoryMapable<S> + 'static, T2: MemoryMapable<S> + 'static, T3: MemoryMapable<S> + 'static> ProgrammInputs for (Binding<S, T1>, Binding<S, T2>, Binding<S, T3>) {
    fn to_raw<'a>(&'a self) -> Self::Raw<'a> {
        (self.0.deref(), self.1.deref(), self.2.deref())
    }

    fn deref_raw<'a, 'c> (&self, raw: &'a Self::Raw<'c>) -> Self::RawDerefed<'a> {
        (&*raw.0, &*raw.1, &*raw.2)
    }
}
//impl<S: Storage, T1: MemoryMapable<S> + 'static, T2: MemoryMapable<S> + 'static, T3: MemoryMapable<S> + 'static> ProgrammInputs for (Binding<S, T1>, Binding<S, T2>, Binding<S, T3>) {}
//impl<S: Storage, T1: MemoryMapable<S> + 'static, T2: MemoryMapable<S> + 'static, T3: MemoryMapable<S> + 'static, T4: MemoryMapable<S> + 'static> ProgrammInputs for (Binding<S, T1>, Binding<S, T2>, Binding<S, T3>, Binding<S, T4>) {}

pub trait ExecutableBindings<S: Storage>{
    type CPU: ExecutableBindings<CPUStorage>;
    type I: ProgrammInputs<Storage = S>;
    type O: MemoryMapable<S>;

    fn get_inputs(&self) -> Self::I;

    fn get_input_references(&self) -> Vec<S::Key>;

    fn get_output(&self) -> Binding<S, Self::O>;

    fn get_out_reference(&self) -> S::Key;

    fn get_layouts() -> HashMap<String, (u8, MemoryLayoutDescriptor, bool)>;

    fn new() -> Self;
}


#[derive(Clone)]
pub struct Binding<S: Storage, T: MemoryMapable<S> + 'static> {
    reference: S::Key,
    storage: S,
    data: Rc<RefCell<S::MappedType<T>>>,
    _0: PhantomData<T>
}

impl<S: Storage, T: MemoryMapable<S> + 'static> Binding<S, T> {
    pub fn new(reference: S::Key, storage: S, data: Rc<RefCell<S::MappedType<T>>>) -> Self {
        Binding { reference: reference, storage, data, _0: PhantomData }
    }

    pub fn deref(&self) -> Ref<S::MappedType<T>> {
        RefCell::borrow(&self.data)
    }

    pub fn get_reference(&self) -> S::Key {
        self.reference.clone()
    }

    pub fn deref_mut(&mut self) -> RefMut<S::MappedType<T>> {
        RefCell::borrow_mut(&self.data)
    }
} 

impl<'a, S: Storage, T: MemoryMapable<S> + 'static> Drop for Binding<S, T> {
    fn drop(&mut self) {
        if self.reference.is_last_reference() {
            self.storage.remove::<S::MappedType<T>>(&self.reference);
        }
    }
}