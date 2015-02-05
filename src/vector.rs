use std::num::Float;
use std::ops::{Add,Mul,Sub};
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
    pub fn add(self, other: Vector) -> Option<Vector> {
        // check for equal sizes
        if self.size != other.size {
            None
        } else {
            let xs: Vec<f64> = self.elements
                                   .iter()
                                   .zip(other.elements.iter())
                                   .map(|(&x, &y)| x + y)
                                   .collect();
            Some(Vector::from_slice(&xs[]))
        }

    }
    pub fn scalar_mul(self, a: f64) -> Vector {
        let xs: Vec<f64> = self.elements
                               .iter()
                               .map(|&x| a * x)
                               .collect();
        Vector {
            elements: xs,
            size: self.size
        }
    }
    pub fn mul(self, other: Vector) -> Option<f64> {
        // check for equal sizes
        if self.size != other.size {
            None
        } else {
            let x: f64 = self.elements
                             .iter()
                             .zip(other.elements.iter())
                             .fold(0.0, |acc, (&x, &y)| acc + x*y);
            Some(x)
        }
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
    type Output = Option<Vector>;

    fn add(self, _rhs: Vector) -> Option<Vector> {
        self.add(_rhs)
    }
}

impl Mul for Vector {
    type Output = Option<f64>;

    fn mul(self, _rhs: Vector) -> Option<f64> {
        self.mul(_rhs)
    }
}

impl Sub for Vector {
    type Output = Option<Vector>;

    fn sub(self, _rhs: Vector) -> Option<Vector> {
        self + _rhs.scalar_mul(-1.0)
    }
}
