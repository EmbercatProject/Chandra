use std::{marker::PhantomData};

use crate::core::{operation::{Operation, OperationWrapper}, type_traits::Iterable, processor::cpu::DifferentiatedCPUContext};

use super::add::Add;

pub fn range<LEFT: Operation<u32>, RIGHT: Operation<u32>>(
    left: LEFT,
    right: RIGHT,
) -> OperationWrapper<u32, Range<LEFT, RIGHT>> {
    OperationWrapper(
        Range {
            left: left,
            right: right
        },
        PhantomData,
    )
}

#[derive(Clone, Debug)]
pub struct Range<LEFT: Operation<u32>, RIGHT: Operation<u32>> {
    pub left: LEFT,
    pub right: RIGHT,
}

impl<LEFT: Operation<u32>, RIGHT: Operation<u32>> Operation<u32> for Range<LEFT, RIGHT> {
    fn evaluate(&self, _context: &mut DifferentiatedCPUContext) -> u32 {
        todo!("Figure out if this needs to be called") //self.left.evaluate(context) / self.right.evaluate(context)
    }
}

impl<LEFT: Operation<u32>, RIGHT: Operation<u32>> Iterable<u32> for Range<LEFT, RIGHT> {
    type StartOp = LEFT;

    type NextOp = OperationWrapper<u32, Add<u32, LEFT, u32>>;

    type BoundryOp = RIGHT;

    fn get_start(&self) -> Self::StartOp {
        return self.left.clone();
    }

    fn get_next(&self) -> Self::NextOp {
        todo!()
    }

    fn get_boundry(&self) -> Self::BoundryOp {
        self.right.clone()
    }
}