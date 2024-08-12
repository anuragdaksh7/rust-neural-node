use ndarray::Array1;
use crate::activation::{activate, Activation};

pub struct Node {
    weights: Array1<f32>,
    bias: f32,
    activation: Activation,
}

impl Node {
    pub fn new(weights: Array1<f32>, bias: f32, activation: Activation) -> Self {
        Node { weights, bias, activation }
    }

    pub fn output(&self, input: &Array1<f32>) -> f32 {
        let output: f32 = activate(&self.activation, self.weights.dot(input) + self.bias);

        output
    }
}








































