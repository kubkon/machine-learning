use std::iter::repeat;
use std::slice::Iter;
use std::fmt::{Display,Formatter,Result};

#[derive(Debug)]
pub struct Vector {
    elements: Vec<f64>,
}

impl Vector {
    pub fn zeros(length: usize) -> Vector {
        Vector {
            elements: repeat(0.0).take(length).collect(),
        }
    }

    pub fn ones(length: usize) -> Vector {
        Vector {
            elements: repeat(1.0).take(length).collect(),
        }
    }

    pub fn from_slice(xs: &[f64]) -> Vector {
        let mut ys = Vec::new();
        ys.push_all(xs);
        Vector {
            elements: ys,
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn iter(&self) -> Iter<f64> {
        self.elements.iter()
    }

    pub fn as_slice(&self) -> &[f64] {
        self.elements.as_slice()
    }

    pub fn get(&self, index: usize) -> f64 {
        self.elements[index]
    }

    pub fn insert(&mut self, index: usize, element: f64) {
        self.elements.insert(index, element);
    }
    
    pub fn scalar_mul(&self, a: f64) -> Vector {
        let xs: Vec<f64> = self.elements
                               .iter()
                               .map(|&x| a * x)
                               .collect();
        Vector {
            elements: xs,
        }
    }

    pub fn add(&self, other: &Vector) -> Vector {
        // check for equal sizes
        assert_eq!(self.elements.len(), other.elements.len());
        let xs: Vec<f64> = self.elements
                               .iter()
                               .zip(other.elements.iter())
                               .map(|(&x, &y)| x + y)
                               .collect();
        Vector::from_slice(&xs)
    }
    
    pub fn sub(&self, other: &Vector) -> Vector {
        self.add(&other.scalar_mul(-1.0))
    }

    pub fn mul(&self, other: &Vector) -> f64 {
        // check for equal sizes
        assert_eq!(self.elements.len(), other.elements.len());
        self.elements
            .iter()
            .zip(other.elements.iter())
            .fold(0.0, |acc, (&x, &y)| acc + x*y)
    }
}

impl Clone for Vector {
    fn clone(&self) -> Vector {
        Vector::from_slice(self.as_slice())
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

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Vector ( {:?} )", self.elements)
    }
}
