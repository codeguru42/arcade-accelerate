use pyo3::prelude::*;

#[pyclass]
pub struct SpatialHash {
    #[pyo3(get, set)]
    cell_size: u32,
}

#[pymethods]
impl SpatialHash {
    #[new]
    fn new(cell_size: u32) -> Self {
        Self { cell_size }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let spatial_hash = SpatialHash { cell_size: 42 };
        assert_eq!(42, spatial_hash.cell_size)
    }
}
