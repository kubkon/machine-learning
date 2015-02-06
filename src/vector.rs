use std::ops::{Add,Mul,Sub};
use std::iter::repeat;
use std::slice::Iter;

#[derive(Debug)]
pub struct Vector<'r> {
    elements: Vec<f64>,
}

impl<'r> Vector<'r> {
    pub fn zeros(length: usize) -> Vector<'r> {
        let xs: Vec<f64> = repeat(0.0).take(length)
                                      .collect();
        Vector {
            elements: xs,
        }
    }

    pub fn ones(length: usize) -> Vector<'r> {
        let mut v = Vector::zeros(length);
        v.elements = v.elements.iter().map(|_| 1.0).collect();
        v
    }

    pub fn from_slice(xs: &[f64]) -> Vector<'r> {
        let mut ys: Vec<f64> = Vec::new();
        ys.push_all(xs);
        Vector {
            elements: ys,
        }
    }

    pub fn len(&'r self) -> usize {
        self.elements.len()
    }

    pub fn iter(&'r self) -> Iter<'r, f64> {
        self.elements.iter()
    }

    pub fn as_slice(&'r self) -> &'r [f64] {
        self.elements.as_slice()
    }

    pub fn get(&'r self, index: usize) -> &'r f64 {
        &self.elements[index]
    }
    
    pub fn scalar_mul(&'r self, a: f64) -> Vector<'r> {
        let xs: Vec<f64> = self.elements
                               .iter()
                               .map(|&x| a * x)
                               .collect();
        Vector {
            elements: xs,
        }
    }

    pub fn _add(&'r self, other: &Vector) -> Option<Vector<'r>> {
        // check for equal sizes
        if self.elements.len() != other.elements.len() {
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

    pub fn _mul(&'r self, other: &Vector) -> Option<f64> {
        // check for equal sizes
        if self.elements.len() != other.elements.len() {
            None
        } else {
            let x: f64 = self.elements
                             .iter()
                             .zip(other.elements.iter())
                             .fold(0.0, |acc, (&x, &y)| acc + x*y);
            Some(x)
        }
    }

    pub fn _sub(&'r self, other: &Vector) -> Option<Vector<'r>> {
        self._add(&other.scalar_mul(-1.0))
    }
}

impl<'r> Clone for Vector<'r> {
    fn clone(&self) -> Vector<'r> {
        Vector::from_slice(self.as_slice())
    }
}

impl<'r> PartialEq for Vector<'r> {
    fn eq(&self, other: &Vector) -> bool {
        self.elements
            .iter()
            .zip(other.elements.iter())
            .all(|(&x, &y)| x == y)
    }
}

impl<'r> Add for Vector<'r> {
    type Output = Option<Vector<'r>>;

    fn add(self, _rhs: Vector) -> Option<Vector<'r>> {
        self._add(&_rhs)
    }
}

impl<'r> Mul for Vector<'r> {
    type Output = Option<f64>;

    fn mul(self, _rhs: Vector) -> Option<f64> {
        self._mul(&_rhs)
    }
}

impl<'r> Sub for Vector<'r> {
    type Output = Option<Vector<'r>>;

    fn sub(self, _rhs: Vector) -> Option<Vector<'r>> {
        self + _rhs.scalar_mul(-1.0)
    }
}
