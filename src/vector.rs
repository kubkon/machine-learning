use std::num::Float;
use std::ops::Add;
use std::iter::repeat;

#[derive(Debug)]
pub struct Vector {
    elements: Vec<f64>,
    size: usize,
}

impl Vector {
    pub fn zeros(length: usize) -> Vector {
        let xs: Vec<f64> = repeat(0.0).take(length)
                                      .collect();
        Vector {
            elements: xs,
            size: length,
        }
    }
    pub fn ones(length: usize) -> Vector {
        let mut v = Vector::zeros(length);
        v.elements = v.elements.iter().map(|_| 1.0).collect();
        v
    }
    pub fn from_slice(xs: &[f64]) -> Vector {
        let mut ys: Vec<f64> = Vec::new();
        ys.push_all(xs);
        Vector {
            elements: ys,
            size: xs.len(),
        }
    }
    pub fn len(self) -> usize {
        self.size
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.elements
            .iter()
            .zip(other.elements.iter())
            .all(|(&x, &y)| x == y)
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, _rhs: Vector) -> Vector {
        // FIX:ME check for equal sizes
        let xs: Vec<f64> = self.elements
                               .iter()
                               .zip(_rhs.elements.iter())
                               .map(|(&x, &y)| x + y)
                               .collect();
        Vector::from_slice(xs.as_slice())
    }
}
