use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
pub struct SpatialHash {
    #[pyo3(get, set)]
    cell_size: i32,
}

#[pymethods]
impl SpatialHash {
    #[new]
    fn new(cell_size: i32) -> PyResult<Self> {
        return if cell_size <= 0 {
            Err(PyValueError::new_err("cell_size must be greater than 0"))
        } else {
            Ok(Self { cell_size })
        };
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
    fn test_hash() {
        let spatial_hash = SpatialHash::new(10).unwrap();
        let point = (20, 30);
        let result = spatial_hash.hash(point);
        let expected = (2, 3);
        assert_eq!(expected, result);
    }
}
