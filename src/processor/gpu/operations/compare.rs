use std::collections::HashMap;

use crate::core::{operations::{equals::Equals, or::Or, and::And, greater_than_or_equal::GreaterThanOrEqual, greater_than::GreaterThan, less_than_or_equal::LessThanOrEqual, less_than::LessThan, not_equal::NotEqual}};

use super::{GPUOperation, GPUComputable};

impl<R: GPUComputable + std::cmp::PartialEq, LEFT: GPUOperation<R>, RIGHT: GPUOperation<R>> GPUOperation<bool> for Equals<R, LEFT, RIGHT> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let left = self.left.build(functions);
        let right = self.right.build(functions);

        format!("{} == {}", left, right)
    }
}

impl<R: GPUComputable + std::cmp::PartialEq, LEFT: GPUOperation<R>, RIGHT: GPUOperation<R>> GPUOperation<bool> for NotEqual<R, LEFT, RIGHT> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let left = self.left.build(functions);
        let right = self.right.build(functions);

        format!("{} != {}", left, right)
    }
}

impl<R: GPUComputable + std::cmp::PartialOrd, LEFT: GPUOperation<R>, RIGHT: GPUOperation<R>> GPUOperation<bool> for LessThan<R, LEFT, RIGHT> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let left = self.left.build(functions);
        let right = self.right.build(functions);

        format!("{} < {}", left, right)
    }
}
impl<R: GPUComputable + std::cmp::PartialOrd, LEFT: GPUOperation<R>, RIGHT: GPUOperation<R>> GPUOperation<bool> for LessThanOrEqual<R, LEFT, RIGHT> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let left = self.left.build(functions);
        let right = self.right.build(functions);

        format!("{} <= {}", left, right)
    }
}

impl<R: GPUComputable + std::cmp::PartialOrd, LEFT: GPUOperation<R>, RIGHT: GPUOperation<R>> GPUOperation<bool> for GreaterThan<R, LEFT, RIGHT> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let left = self.left.build(functions);
        let right = self.right.build(functions);

        format!("{} > {}", left, right)
    }
}
impl<R: GPUComputable + std::cmp::PartialOrd, LEFT: GPUOperation<R>, RIGHT: GPUOperation<R>> GPUOperation<bool> for GreaterThanOrEqual<R, LEFT, RIGHT> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let left = self.left.build(functions);
        let right = self.right.build(functions);

        format!("{} >= {}", left, right)
    }
}


impl<LEFT: GPUOperation<bool>, RIGHT: GPUOperation<bool>> GPUOperation<bool> for And<LEFT, RIGHT> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let left = self.left.build(functions);
        let right = self.right.build(functions);

        format!("{} && {}", left, right)
    }
}

impl<LEFT: GPUOperation<bool>, RIGHT: GPUOperation<bool>> GPUOperation<bool> for Or<LEFT, RIGHT> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let left = self.left.build(functions);
        let right = self.right.build(functions);

        format!("{} || {}", left, right)
    }
}