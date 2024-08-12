use ndarray::array;
use crate::activation::Activation;
use crate::node::Node;

mod activation;
mod node;

fn main() {
    let weights = array![1.0, 2.0, 3.0];
    let node = Node::new(weights, -2.3, Activation::Sigmoid);
    let inputs = array![1.0, 2.0, 3.0];
    let out = node.output(&inputs);
    println!("{out}")
}
