use std::{marker::PhantomData, fmt::Debug};

use super::{operation::{Operation, Differentiable}, type_traits::{Generalizable, IndexAble, GetAndSetable}, allocated::MemoryLayoutDescriptor, processor::cpu::DifferentiatedCPUContext};

pub type Shape = Vec<usize>;

#[derive(Clone, Debug)]
pub enum Either<A: Clone, B: Clone> {
    A(A),
    B(B)
}

impl<R: Value, A: Operation<R>, B: Operation<R>> Operation<R> for Either<A, B> {
    fn evaluate(&self, context: &mut DifferentiatedCPUContext) -> R {
        match self {
            Either::A(x) => x.evaluate(context),
            Either::B(x) => x.evaluate(context),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExternalWrapper<T: Generalizable> {
    name: String,
    _0: PhantomData<T>
}

impl<T: Generalizable> ExternalWrapper<T> {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), _0: PhantomData }
    }
}

impl<T: Generalizable> IndexAble for ExternalWrapper<T> where T: IndexAble {
    type IndexResult = T::IndexResult;

    fn index<O: Operation<u32>>(&self, _index: O) -> Self::IndexResult {
        todo!()
    }

    fn get_reference(&self) -> String {
        todo!()
    }

    fn get_field() -> Option<String> {
        todo!()
    }
    
}

impl<T: Generalizable, R: Computable> GetAndSetable<R> for ExternalWrapper<T> where T: GetAndSetable<R> {
    fn get_variable(&self) -> Option<String> {
        todo!()
    }
}

pub trait Value: Clone + Copy + Send + Sync + Debug {
    fn get_val_type() -> &'static str;
}

impl<C: Computable> Differentiable<C> for C {
    type Diff = C;

    fn auto_diff_for<R1: Clone>(&self, _var: super::operations::var::Variable<R1>, _var_trace: &mut std::collections::HashMap<String, Vec<String>>) -> Self::Diff {
        C::get_zero()
    }

    fn contains_var<R1: Clone>(&self, _var: super::operations::var::Variable<R1>) -> bool {
        false
    }
}

pub trait Computable: Value {
    type Type: Computable;
    fn get_type() -> &'static str;
    fn get_value(val: Self) -> Self;
    fn get_zero() -> Self;
    fn from_int(from: isize) -> Self;
    fn from_float(from: f64) -> Self;
    fn byte_size() -> usize;
    fn get_memory_layout() -> MemoryLayoutDescriptor;
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: Vec<u8>) -> Self;
}

impl<C: Computable> Value for C {
    fn get_val_type() -> &'static str {
        C::get_type()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pos {pub x: u32, pub y: u32, pub z: u32}

impl<R: Computable> Operation<R> for R {
    fn evaluate(&self, _context: &mut super::processor::cpu::DifferentiatedCPUContext) -> R {
        *self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Void;
impl Value for Void {
    fn get_val_type() -> &'static str {
        "void"
    }
}

impl Operation<Void> for Void {
    fn evaluate(&self, _context: &mut super::processor::cpu::DifferentiatedCPUContext) -> Void {
        Void
    }
}

impl Computable for bool {
    type Type = Self;

    fn get_type() -> &'static str {
        "bool"
    }

    fn get_value(val: Self) -> Self {
        val
    }

    fn get_zero() -> Self {
        false
    }

    fn from_int(from: isize) -> Self {
        if from == 0 {
            false
        } else {
            true
        }
    }

    fn from_float(from: f64) -> Self {
        (1.0 - from).abs() < 0.01
    }

    fn byte_size() -> usize {
        1
    }

    fn get_memory_layout() -> MemoryLayoutDescriptor {
        MemoryLayoutDescriptor::UInteger(1)
    }

    fn to_bytes(&self) -> Vec<u8> {
        if *self {
            vec![1]
        } else {
            vec![0]
        }
    }

    fn from_bytes(bytes: Vec<u8>) -> Self {
        if bytes[0] == 1 {
            true
        } else {
            false
        }
    }
}

impl Computable for f32 {
    type Type = Self;

    fn get_type() -> &'static str {
        "f32"
    }

    fn get_value(val: Self) -> Self {
        val
    }

    fn get_zero() -> Self {
        0_f32
    }

    fn from_int(from: isize) -> Self {
       from as f32
    }

    fn from_float(from: f64) -> Self {
       from as f32
    }

    fn byte_size() -> usize {
        4
    }

    fn get_memory_layout() -> MemoryLayoutDescriptor {
        MemoryLayoutDescriptor::Float(4)
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().into()
    }

    fn from_bytes(bytes: Vec<u8>) -> Self {
        f32::from_le_bytes(bytes.try_into().unwrap())
    }
}

impl Computable for i32 {
    type Type = Self;

    fn get_type() -> &'static str {
        "i32"
    }

    fn get_value(val: Self) -> Self {
        val
    }

    fn get_zero() -> Self {
        0
    }
    fn from_int(from: isize) -> Self {
        from as i32
     }
 
     fn from_float(from: f64) -> Self {
        from as i32
     }

    fn byte_size() -> usize {
        4
    }

    fn get_memory_layout() -> MemoryLayoutDescriptor {
        MemoryLayoutDescriptor::Integer(4)
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().into()
    }

    fn from_bytes(bytes: Vec<u8>) -> Self {
        i32::from_le_bytes(bytes.try_into().unwrap())
    }
}

impl Computable for i64 {
    type Type = Self;

    fn get_type() -> &'static str {
        "i64"
    }

    fn get_value(val: Self) -> Self {
        val
    }

    fn get_zero() -> Self {
        0
    }
    fn from_int(from: isize) -> Self {
        from as i64
     }
 
     fn from_float(from: f64) -> Self {
        from as i64
     }

    fn byte_size() -> usize {
        8
    }

    fn get_memory_layout() -> MemoryLayoutDescriptor {
        MemoryLayoutDescriptor::Integer(8)
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().into()
    }

    fn from_bytes(bytes: Vec<u8>) -> Self {
        i64::from_le_bytes(bytes.try_into().unwrap())
    }
}

impl Computable for u32 {
    type Type = Self;

    fn get_type() -> &'static str {
        "u32"
    }

    fn get_value(val: Self) -> Self {
        val
    }

    fn get_zero() -> Self {
        0
    }
    fn from_int(from: isize) -> Self {
        from as u32
     }
 
     fn from_float(from: f64) -> Self {
        from as u32
     }

    fn byte_size() -> usize {
        4
    }

    fn get_memory_layout() -> MemoryLayoutDescriptor {
        MemoryLayoutDescriptor::UInteger(4)
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().into()
    }

    fn from_bytes(bytes: Vec<u8>) -> Self {
        u32::from_le_bytes(bytes.try_into().unwrap())
    }
}

impl Computable for u64 {
    type Type = Self;

    fn get_type() -> &'static str {
        "u64"
    }

    fn get_value(val: Self) -> Self {
        val
    }

    fn get_zero() -> Self {
        0
    }
    fn from_int(from: isize) -> Self {
        from as u64
     }
 
     fn from_float(from: f64) -> Self {
        from as u64
     }

    fn byte_size() -> usize {
        8
    }

    fn get_memory_layout() -> MemoryLayoutDescriptor {
        MemoryLayoutDescriptor::UInteger(8)
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().into()
    }

    fn from_bytes(bytes: Vec<u8>) -> Self {
        u64::from_le_bytes(bytes.try_into().unwrap())
    }
}
