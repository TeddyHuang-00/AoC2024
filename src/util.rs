pub fn mean(values: &[f64]) -> f64 {
    let sum = values.iter().sum::<f64>();
    let count = values.len();
    sum / count as f64
}

pub fn std_dev(values: &[f64]) -> f64 {
    let mean = mean(values);
    let sum = values.iter().map(|v| (*v - mean).powi(2)).sum::<f64>();
    let count = values.len();
    (sum / count as f64).sqrt()
}
