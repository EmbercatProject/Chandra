use std::{cell::{RefCell, Ref, RefMut}, rc::{Rc}, marker::PhantomData, any::Any};

use rayon::prelude::IntoParallelIterator;

use crate::core::{any_map::AnyMap, processor::{Storage, cpu::CPUFunction, Executable, ProcessorInformation, Processor}, allocated::{ExecutableBindings, Binding, ProgrammInputs}, Buildable, types::{Void, Pos}, type_traits::MemoryMapable};
use crate::core::type_traits::FromMut;

use rayon::iter::ParallelIterator;

pub struct CPUProcessor {
    pub heap: CPUStorage,
    pub counter: usize,
}

pub struct CPUExecutable<B: ExecutableBindings<CPUStorage>, Fn: CPUFunction<B>, Bu: Buildable<CPUProcessor>> {
    pub function: Fn,
    bindings: B,
    _0: PhantomData<Bu>
}

impl<B: ExecutableBindings<CPUStorage>, Fn: CPUFunction<B>, Bu: Buildable<CPUProcessor>> Executable<CPUStorage, B> for CPUExecutable<B, Fn, Bu> {
    fn get_bindings(&mut self) -> &mut B {
        &mut self.bindings
    }
    fn get_bindings_ref(&self) -> & B {
        &self.bindings
    }
}

#[derive(Clone)]
pub struct CPUStorage(Rc<RefCell<AnyMap<usize>>>);

impl CPUStorage {
    pub fn new() -> Self {
        CPUStorage(Rc::new(RefCell::new(AnyMap::<usize>::new())))
    }

    fn insert<V: std::any::Any>(&mut self, key: &<CPUStorage as Storage>::Key, val: V) -> Option<V> {
        self.0.borrow_mut().insert(**key, val)
    }

    fn get<V: std::any::Any>(&self, key: &<CPUStorage as Storage>::Key) -> Option<Ref<V>> {
        let borrowed = self.0.borrow();

        borrowed.get::<V>(**key)?;

        Some(Ref::map(borrowed, |x| x.get::<V>(**key).unwrap()))
    }

    fn get_mut<V: std::any::Any>(&mut self, key: &<CPUStorage as Storage>::Key) -> Option<RefMut<V>> {
        let borrowed = self.0.borrow_mut();

        borrowed.get::<V>(**key)?;

        Some(RefMut::map(borrowed, |x| x.get_mut::<V>(**key).unwrap()))
    }
}

impl Storage for CPUStorage {
    type Key = Rc<usize>;
    type Data<T> = Rc<RefCell<T>>;
    type MappedType<T: Any + Clone + Send + Sync> = T;

    fn remove<V: std::any::Any + Clone + Send + Sync>(&mut self, key: &Self::Key) -> Option<Self::MappedType<V>> {
        self.0.borrow_mut().remove::<Self::MappedType<V>>(**key)
    }
}


impl ProcessorInformation for CPUProcessor {
    type Storage = CPUStorage;
    type Executable<B: Buildable<Self>> = CPUExecutable<<B as Buildable<Self>>::CPUBinding, B::CPUFunction, B>;
    type Compiler = Void;
}

impl Processor for CPUProcessor {
    
    fn build<B: Buildable<Self>>(&mut self, buildable: B) -> Self::Executable<B> {
        CPUExecutable {
            function: buildable.get_cpu(),
            bindings: B::CPUBinding::new(),
            _0: PhantomData
        }
    }

    fn alloc<T: MemoryMapable<Self::Storage>>(&mut self, val: T) -> Binding<Self::Storage, T> {
        let reference = Rc::new(self.counter);
        self.counter += 1;

        let mapped = val.into_processor_mapped();
        let refered = Rc::new(RefCell::new(mapped));

        let binding = Binding::new(reference.clone(), self.heap.clone(), refered.clone());

        self.heap.insert(&reference, refered);
        
        binding
    }

    fn dispatch<'a, B: Buildable<Self>>(&'a mut self, executable: &'a mut Self::Executable<B>, x: u32, y: u32, z: u32)
        where <B::Binding as ExecutableBindings<Self::Storage>>::O: 'static, <B::CPUBinding as ExecutableBindings<Self::Storage>>::O: 'static {
            let bindings = executable.get_bindings_ref();

            let mut outpt = bindings.get_output();
    
            let inputs = bindings.get_inputs();
    
            let raw = inputs.to_raw();
            
            let inp = inputs.deref_raw(&raw);
            let mut out_raw = outpt.deref_mut();
            let out: &mut <<B as Buildable<CPUProcessor>>::CPUBinding as ExecutableBindings<CPUStorage>>::O = &mut *out_raw;
            let o = <<<B as Buildable<CPUProcessor>>::CPUBinding as ExecutableBindings<CPUStorage>>::O as MemoryMapable<CPUStorage>>::Mapped::from_mut(out);

            let pos = Pos {x: 0, y: 0, z: 0};

            let cores = rayon::current_num_threads();

            (0..cores)
                .into_iter()
                .map(|i| (i, o.clone()))
                .collect::<Vec<(usize, _)>>()
                .into_par_iter()
                .for_each(|(i, mut o)| {
                    let mut pos = pos.clone();
                    let func = executable.function.clone();
                    let iterations = if x % (cores as u32) > 0 {x / (cores as u32) + 1} else {x / (cores as u32)};

                    for x in ((i as u32) * iterations)..((i as u32 +1 as u32)*iterations) {
                        pos.x = x;
                        for y in 0..y {
                            pos.y = y;
                            for z in 0..z {
                                pos.z = z;
            
                                func.call_cpu(&pos, &inp, &mut o);
                            }
                        }

                    }
                });
    
            //for x in 0..x {
            //    pos.x = x;
            //    for y in 0..y {
            //        pos.y = y;
            //        for z in 0..z {
            //            pos.z = z;
    //
            //            executable.function.call_cpu(&pos, &inp, &mut o);
            //        }
            //    }
            //}
    }

    fn dealloc<T: MemoryMapable<Self::Storage>>(&mut self, val: Binding<Self::Storage, T>) -> T {
        
        let v = {
            self.heap.remove::<<CPUStorage as Storage>::MappedType<T>>(&val.get_reference())
        };

        v.unwrap()
    }

    fn copy_to_cpu<T: MemoryMapable<Self::Storage>>(&mut self, val: &Binding<Self::Storage, T>) -> T {
        self.heap.get::<T>(&val.get_reference()).unwrap().clone()
    }

   
}
