use std::{collections::HashMap};

use crate::core::{operations::{add::Add, divide::Divide, multiply::Multiply, subtract::Subtract}, type_traits::Calculatable};

use super::{GPUOperation, GPUComputable};

impl<R: Calculatable + GPUComputable, LEFT: GPUOperation<R>, RIGHT: GPUOperation<R>> GPUOperation<R> for Add<R, LEFT, RIGHT> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let left = self.left.build(functions);
        let right = self.right.build(functions);

        format!("{} + {}", left, right)
    }
}
impl<R: Calculatable + GPUComputable, LEFT: GPUOperation<R>, RIGHT: GPUOperation<R>> GPUOperation<R> for Divide<R, LEFT, RIGHT> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let left = self.left.build(functions);
        let right = self.right.build(functions);

        format!("{} / {}", left, right)
    }
}
impl<R: Calculatable + GPUComputable, LEFT: GPUOperation<R>, RIGHT: GPUOperation<R>> GPUOperation<R> for Multiply<R, LEFT, RIGHT> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let left = self.left.build(functions);
        let right = self.right.build(functions);

        format!("{} * {}", left, right)
    }
}
impl<R: Calculatable + GPUComputable, LEFT: GPUOperation<R>, RIGHT: GPUOperation<R>> GPUOperation<R> for Subtract<R, LEFT, RIGHT> {
    fn build(&self, functions: &mut HashMap<String, String>) -> String {
        let left = self.left.build(functions);
        let right = self.right.build(functions);

        format!("{} - {}", left, right)
    }
}