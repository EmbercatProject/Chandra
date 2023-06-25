use crate::core::{operation::{Operation, Differentiable}, types::Void, processor::cpu::DifferentiatedCPUContext};


#[derive(Clone, Debug)]
pub struct Noop;

impl Operation<Void> for Noop {
    fn evaluate(&self, _context: &mut DifferentiatedCPUContext) -> Void {
        Void
    }
}

impl Differentiable<Void> for Noop {
    type Diff = Noop;

    fn auto_diff_for<R1: Clone>(&self, _var: super::var::Variable<R1>, _var_trace: &mut std::collections::HashMap<String, Vec<String>>) -> Self::Diff {
        Noop
    }

    fn contains_var<R1: Clone>(&self, _var: super::var::Variable<R1>) -> bool {
        false
    }
}

/* 
impl GPUInstruction<Void> for Noop {
    fn build_gpu(&self, _: &mut GPU) -> String {
        return "".into();
    }
}

impl<E: EnvironmentStorage> CPUInstruction<Void, E> for Noop {
    fn build_cpu(&self, _: &mut CPU<E>) -> Box<dyn Fn(&mut CPU<E>) -> Void> {
        return Box::new(|_| Void )
    }
}
*/