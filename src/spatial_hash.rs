use pyo3::prelude::*;

#[pyclass]
pub struct SpatialHash {
    #[pyo3(get, set)]
    num: i32
}

#[pymethods]
impl SpatialHash {
    #[new]
    fn new(value: i32) -> Self {
        SpatialHash { num: value }
    }
}
