use pyo3::prelude::*;

#[pyclass]
pub struct SpatialHash {
    #[pyo3(get, set)]
    cell_size: u16,
}

#[pymethods]
impl SpatialHash {
    #[new]
    fn new(cell_size: u16) -> Self {
        Self { cell_size }
    }

    fn hash(&self, point: (i32, i32)) -> (i32, i32) {
        (
            point.0 / i32::from(self.cell_size),
            point.1 / i32::from(self.cell_size),
        )
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

    #[test]
    fn test_hash() {
        let spatial_hash = SpatialHash { cell_size: 10 };
        let point = (20, 30);
        let result = spatial_hash.hash(point);
        let expected = (2, 3);
        assert_eq!(expected, result);
    }
}
