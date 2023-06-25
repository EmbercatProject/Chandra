use std::{marker::PhantomData, sync::{Mutex, Arc}, rc::Rc};

use super::{type_traits::{ProgammInputs, Generalizable}, Procedure, operation::Operation, types::{Void, Computable, Shape, Pos}, tensor::{GenericTensor, Tensor, CPUTensor, VmTensor}, any_map::AnyMap};

pub trait Executable<I: ProgammInputs, O: Generalizable> {
    type Processor: Processor;
    fn run(&self, processor: &mut Self::Processor, inputs: &I, output: &mut O);

    fn bind(&self, i1: I, output: &mut O) -> (&Self, Binding<I,O>);
}

pub struct Binding<I: ProgammInputs, O: Generalizable> {
    inputs: I,
    output: O
}

pub struct VMExecutable<I: ProgammInputs, O: Generalizable> {
    _0: PhantomData<I>,
    _1: PhantomData<O>,
}
impl<I: ProgammInputs, O: Generalizable> Executable<I,O> for VMExecutable<I,O> {
    type Processor = VMProcessor;

    fn run(&self, processor: &mut Self::Processor, inputs: &I, output: &mut O) {
        todo!()
    }

    fn bind(&self, i1: I, output: &mut O) -> (&Self, Binding<I,O>) {
        todo!()
    }
}


pub struct CPUExecutable<I: ProgammInputs, O: Generalizable> {
    function: Box<dyn (Fn(Pos, &I, &mut O) -> ())>,
}
impl<I: ProgammInputs, O: Generalizable> Executable<I,O> for CPUExecutable<I,O> {
    type Processor = CPUProcessor;

    fn run(&self, processor: &mut Self::Processor, inputs: &I, output: &mut O) {
        todo!()
    }

    fn bind(&self, i1: I, output: &mut O) -> (&Self, Binding<I,O>) {
        todo!()
    }
} 


pub trait Processor where Self: Sized {
    type Target<I: ProgammInputs, O: Generalizable>: Executable<I,O>;
    type Tensor<T: Computable>: GenericTensor<T>;

    fn build<S: Operation<Void>, I: ProgammInputs, O: Generalizable>(&mut self, procedure: Procedure<S,I,O>) -> Self::Target<I::ExactType<Self>,O::ExactType<Self>>;
    fn tensor<T: Computable>(&mut self, val: T, shape: Shape) -> Tensor<T, Self::Tensor<T>>;
    fn dispatch<I: ProgammInputs, O: Generalizable>(&mut self, executable: (&Self::Target<I, O>, Binding<I,O> ), x: u64, y: u64, z: u64);
}


#[derive(Clone)]
pub struct VMProcessor {

}

impl Processor for VMProcessor {
    type Target<I: ProgammInputs, O: Generalizable> = VMExecutable<I,O>;
    type Tensor<T: Computable> = VmTensor<T>;

    fn build<S: Operation<Void>, I: ProgammInputs, O: Generalizable>(&mut self, procedure: Procedure<S,I,O>) -> Self::Target<I::ExactType<Self>,O::ExactType<Self>> {
        todo!()
    }

    fn tensor<T: Computable>(&mut self, val: T, shape: Shape) -> Tensor<T, Self::Tensor<T>> {
        todo!()
    }

    fn dispatch<I: ProgammInputs, O: Generalizable>(&mut self, (executable, binding): (&Self::Target<I, O>, Binding<I,O> ), x: u64, y: u64, z: u64) {
        todo!()
    }
}

pub struct CPUProcessor {
    pub values: Rc<Mutex<AnyMap<usize>>>,
    pub counter: usize
}

#[derive(Clone)]
pub struct Tens<T: Computable + 'static> {
    pub reference: Rc<usize>,
    _map: Rc<Mutex<AnyMap<usize>>>,
    _0: PhantomData<T> 
}

impl<T: Computable + 'static> Drop for Tens<T> {
    fn drop(&mut self) {
        if 1 == Rc::<usize>::strong_count(&self.reference) {
            let mut lock = self._map.lock().unwrap();
            lock.remove::<CPUTensor<T>>(*self.reference);
        }
    }
}

impl CPUProcessor {
    pub fn new_tensor<T: Computable + 'static>(&mut self, val: T, shape: Shape) -> Tens<T> {
        let local = CPUTensor {
            data: vec![val; shape.clone().into_iter().product()],
            shape,
        };

        let key = self.counter;
        self.counter += 1;

        {
            let mut vals = self.values.lock().unwrap();
            vals.insert(key, local);
        }

        

        Tens { reference: Rc::new(key), _map: self.values.clone(), _0: PhantomData }
    }

    pub fn do_stuff<T: Computable + 'static>(&mut self, tens: &Tens<T>) {
        let mut vals = self.values.lock().unwrap();
        let cpu_tensor = vals.get_mut::<CPUTensor<T>>(*tens.reference).unwrap();
        let cpu_tensor2 = vals.get::<CPUTensor<T>>(*tens.reference).unwrap();
        
    }
}

impl Processor for CPUProcessor {
    type Target<I: ProgammInputs, O: Generalizable> = CPUExecutable<I,O>;
    type Tensor<T: Computable> = CPUTensor<T>;

    fn build<S: Operation<Void>, I: ProgammInputs, O: Generalizable>(&mut self, procedure: Procedure<S,I,O>) -> Self::Target<I::ExactType<Self>,O::ExactType<Self>> {
        return CPUExecutable {
            function: procedure.cpu_function
        }
    }

    fn tensor<T: Computable>(&mut self, val: T, shape: Shape) -> Tensor<T, Self::Tensor<T>> {
        return Tensor {
            data: CPUTensor {
                data: vec![val; shape.iter().product()],
                shape,
            },
            _0: PhantomData,
        }
    }

    fn dispatch<I: ProgammInputs, O: Generalizable>(&mut self, (executable, binding): (&Self::Target<I, O>, Binding<I,O> ), x: u64, y: u64, z: u64) {
        todo!()
    }
}