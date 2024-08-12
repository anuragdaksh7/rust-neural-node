#[allow(dead_code)]
pub enum Activation {
    Sigmoid,
    ReLU,
    Tanh,
}

pub fn activate(activation: &Activation, x: f32) -> f32 {
    match activation {
        Activation::ReLU => relu(x),
        Activation::Tanh => tanh(x),
        Activation::Sigmoid => sigmoid(x),
    }
}

pub fn relu(x: f32) -> f32 {
    if x>0.0 {
        x
    } else {
        0.0
    }
}

pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

pub fn tanh(x: f32) -> f32 {
    let exp_2x = (2.0 * x).exp();
    (exp_2x - 1.0) / (exp_2x + 1.0)
}
