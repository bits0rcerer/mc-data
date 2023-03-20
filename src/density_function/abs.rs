use std::sync::Arc;

use crate::density_function::DensityFunction;
use crate::density_function::transformer::Transformer;

pub fn abs(f: Box<dyn DensityFunction>) -> Box<dyn DensityFunction> {
    Transformer::new(f, Arc::new(|x: f64| x.abs()))
}
