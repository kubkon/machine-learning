use std::iter::repeat;
use std::slice::Iter;
use std::fmt::{Display,Formatter,Result};

#[derive(Debug)]
pub struct Vector<'r> {
    elements: Vec<f64>,
}

impl<'r> Vector<'r> {
    pub fn zeros(length: usize) -> Vector<'r> {
        Vector {
            elements: repeat(0.0).take(length).collect(),
        }
    }

    pub fn ones(length: usize) -> Vector<'r> {
        Vector {
            elements: repeat(1.0).take(length).collect(),
        }
    }

    pub fn from_slice(xs: &[f64]) -> Vector<'r> {
        let mut ys = Vec::new();
        ys.push_all(xs);
        Vector {
            elements: ys,
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn iter(&'r self) -> Iter<'r, f64> {
        self.elements.iter()
    }

    pub fn as_slice(&'r self) -> &'r [f64] {
        self.elements.as_slice()
    }

    pub fn get(&self, index: usize) -> f64 {
        self.elements[index]
    }
    
    pub fn scalar_mul(&self, a: f64) -> Vector<'r> {
        let xs: Vec<f64> = self.elements
                               .iter()
                               .map(|&x| a * x)
                               .collect();
        Vector {
            elements: xs,
        }
    }

    pub fn add(&self, other: &Vector) -> Option<Vector<'r>> {
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
    
    pub fn sub(&self, other: &Vector) -> Option<Vector<'r>> {
        self.add(&other.scalar_mul(-1.0))
    }

    pub fn mul(&self, other: &Vector) -> Option<f64> {
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

impl<'r> Display for Vector<'r> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Vector ( {:?} )", self.elements)
    }
}