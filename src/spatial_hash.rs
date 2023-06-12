use std::collections::HashMap;

use crate::BasicSprite;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
pub struct SpatialHash {
    #[pyo3(get, set)]
    cell_size: i32,
    contents: HashMap<(i32, i32), Vec<&BasicSprite>>,
    buckets_for_sprite: HashMap<BasicSprite, Vec<Vec<&BasicSprite>>>,
}

#[pymethods]
impl SpatialHash {
    #[new]
    fn new(cell_size: i32) -> PyResult<Self> {
        return if cell_size <= 0 {
            Err(PyValueError::new_err("cell_size must be greater than 0"))
        } else {
            Ok(Self {
                cell_size,
                contents: HashMap::new(),
                buckets_for_sprite: HashMap::new(),
            })
        };
    }

    fn hash(&self, point: (i32, i32)) -> (i32, i32) {
        (
            point.0 / i32::from(self.cell_size),
            point.1 / i32::from(self.cell_size),
        )
    }

    fn add(&mut self, sprite: &BasicSprite) {
        let min_point = (sprite.native_left() as i32, sprite.native_bottom() as i32);
        let max_point = (sprite.native_right() as i32, sprite.native_top() as i32);
        let min_hash = self.hash(min_point);
        let max_hash = self.hash(max_point);
        let mut buckets: Vec<Vec<&BasicSprite>> = Vec::new();

        for i in min_hash.0..max_hash.0 {
            for j in min_hash.1..max_hash.1 {
                let bucket = self.contents.entry((i, j)).or_insert(Vec::new());
                bucket.push(&sprite);
                buckets.push(bucket.clone());
            }
        }

        self.buckets_for_sprite.insert(sprite, buckets);
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

    #[test]
    fn test_add() {
        let mut spatial_hash = SpatialHash::new(10).unwrap();
        let sprite = BasicSprite::new();
        spatial_hash.add(&sprite)
    }
}
