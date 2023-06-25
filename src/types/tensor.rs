use std::{ops::{Index, IndexMut, Add}, collections::HashMap, sync::Arc};

use parking_lot::RwLock;
use rand::prelude::Distribution;

use crate::{processor::{cpu::CPUStorage, gpu::processor::GPUStorage}, core::{types::{Computable, Shape}, operations::{add::Add as OAdd, self}, operation::{Operation, OperationWrapper}, type_traits::{IndexAble, Generalizable, MemoryMapable, FromMut, Calculatable}, processor::{Storage}, allocated::{MemoryLayoutDescriptor, ParallelizationDescriptor, StructParallelizationDescriptor, Parallelizable}, guards::region_guard::RegionGuard}};

#[derive(Clone, Debug)]
pub struct Tensor<C: Computable> {
    pub data: Vec<C>,
    pub shape: Shape
}

impl<C: Computable> Tensor<C> {
    pub fn new(val: C, shape: Shape) -> Self {
        Self {
            data: vec![val; shape.iter().product()],
            shape
        }
    }
    pub fn rand(shape: Shape) -> Tensor<f32> {
        let dist = rand::distributions::Uniform::new(-1.0, 1.0);
        let mut rng = rand::thread_rng();
        let data: Vec<f32> = (0..shape.iter().product())
            .into_iter()
            .map(|_| dist.sample(&mut rng))
            .collect();

        Tensor::<f32> {
            data: data,
            shape
        }
    }
}

#[derive(Clone)]
pub struct MyCPUTensor<'a, C: Computable> {
    shape: Arc<RwLock<&'a Vec<usize>>>,
    data: RegionGuard<'a, C>
}

impl<'a, C: Computable> Index<u32> for MyCPUTensor<'a, C> {
    type Output = C;

    fn index(&self, index: u32) -> &Self::Output {
        self.data.index(index as usize)
    }
}

impl<'a, C: Computable> Index<(u32, u32)> for MyCPUTensor<'a, C> {
    type Output = C;

    fn index(&self, (region, entry): (u32, u32)) -> &Self::Output {
        self.data.index_2((region as usize, entry as usize))
    }
}

impl<'a, C: Computable> IndexMut<u32> for MyCPUTensor<'a, C> {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        self.data.index_mut(index as usize)
    }
}

impl<'a, C: Computable> IndexMut<(u32,u32)> for MyCPUTensor<'a, C> {
    fn index_mut(&mut self, (region, entry): (u32, u32)) -> &mut Self::Output {
        self.data.index_mut_2((region as usize, entry as usize))
    }
}

// TODO: Make correct and shape independent conversion (Check memory alignment)
impl<'a, C: Computable + 'static> FromMut<Tensor<C>> for MyCPUTensor<'a, C> {
    type Result<'b> = MyCPUTensor<'b, C>;

    fn from_mut<'b>(value: &'b mut Tensor<C>) -> Self::Result<'b> {
        let mut current: usize = 0;
        let mut regions = Vec::new();

        for _i in 0..value.shape[0] {
            let end = value.shape[1..].iter().product::<usize>();
            regions.push((current, current + end));
            current += end;
        }

        MyCPUTensor { 
            shape: Arc::new(RwLock::new(&value.shape)), 
            data: RegionGuard::new(&mut value.data, regions)
        }
    }
}

impl<C: Computable + 'static> MemoryMapable<CPUStorage> for Tensor<C> {
    type Mapped<'a> = MyCPUTensor<'a, C>;

    fn get_struct_map(&self) -> std::collections::HashMap<String, String> {
        HashMap::new()
    }

    fn to_memory_bytes(&self) -> Vec<u8> {
        
        vec![0_u8; 1]
    }

    fn from_memory_bytes(_bytes: Vec<u8>) -> Self {
        todo!()
    }

    fn into_processor_mapped(self) -> <CPUStorage as Storage>::MappedType<Self> {
        self
    }
    
}

impl<C: Computable + 'static> MemoryMapable<GPUStorage> for Tensor<C> {
    type Mapped<'a> = usize;

    fn get_struct_map(&self) -> std::collections::HashMap<String, String> {
        HashMap::new()
    }

    fn to_memory_bytes(&self) -> Vec<u8> {
        
        vec![0_u8; 1]
    }

    fn from_memory_bytes(_bytes: Vec<u8>) -> Self {
        todo!()
    }

    fn into_processor_mapped(self) -> <GPUStorage as Storage>::MappedType<Self> {
        0
    }
    
}


impl<C: Computable> Generalizable for Tensor<C> {
    fn get_memory_layout() -> MemoryLayoutDescriptor {
        return MemoryLayoutDescriptor::Struct(
            HashMap::from([
                ("shape".to_string(), (0, MemoryLayoutDescriptor::Vector { item_typ: Box::new(u64::get_memory_layout()) })),
                ("data".to_string(), (1, MemoryLayoutDescriptor::Vector { item_typ: Box::new(C::get_memory_layout()) }))
            ]));
    }

    fn get_parallelization_info(&self) -> ParallelizationDescriptor {
        ParallelizationDescriptor::Struct(
            StructParallelizationDescriptor {
                this: Parallelizable::X(self.shape[0] as u64),
                fields: HashMap::from([
                    ("shape".to_string(), (0, MemoryLayoutDescriptor::Vector { item_typ: Box::new(u64::get_memory_layout()) }, ParallelizationDescriptor::Data(Parallelizable::Sync))),
                    ("data".to_string(), (1, MemoryLayoutDescriptor::Vector { item_typ: Box::new(C::get_memory_layout()) }, ParallelizationDescriptor::Data(Parallelizable::X(self.shape[0] as u64)))),
                ]),
            }
        )
    }
}

impl<C: Computable> IndexAble for Tensor<C> {
    type IndexResult = C;

    fn index<O: Operation<u32>>(&self, _index: O) -> Self::IndexResult {
        todo!()
    }

    fn get_reference(&self) -> String {
        unimplemented!()
    }

    fn get_field() -> Option<String> {
        Some("data".to_string())
    }
    
}

impl<T: Computable> BaseTensor for Tensor<T> {
    fn get_type(&self) -> &str {
        T::get_type()
    }
}

impl<C: Computable> Index<u32> for Tensor<C> {
    type Output = C;

    fn index(&self, index: u32) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl<C: Computable> IndexMut<u32> for Tensor<C> {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}

pub trait BaseTensor {
    fn get_type(&self) -> &str;
}

pub trait GenericTensor<C: Computable>: BaseTensor + Clone {
    fn get_shape(&self) -> &Shape;
}

impl<R: Calculatable, LEFT: Operation<R>, RIGHT: Operation<R>> Add<OperationWrapper<R, RIGHT>> for OperationWrapper<R, LEFT> {
    type Output = OperationWrapper<R, OAdd<R, LEFT, RIGHT>>;

    fn add(self, rhs: OperationWrapper<R, RIGHT>) -> Self::Output {
        operations::add::add(self.0, rhs.0)
    }
}