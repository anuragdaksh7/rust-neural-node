use ndarray::Array1;

pub fn normalize_array(arr: &Array1<f32>) -> Array1<f32> {
    let norm = arr.norm_l2();
    arr / norm
}