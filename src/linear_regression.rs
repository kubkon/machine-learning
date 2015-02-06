use std::num::Float;
use vector::Vector;

#[derive(Debug)]
pub struct LinearRegression<'r> {
    pub step: f64,
    pub tolerance: f64,
    pub params: Vector<'r>,
}

impl<'r> LinearRegression<'r> {
    pub fn new(step: f64, tolerance: f64) -> LinearRegression<'r> {
        LinearRegression {
            step: step,
            tolerance: tolerance,
            params: Vector::zeros(2),
        }
    }

    fn model_func(&self, xs: &Vector) -> Vector<'r> {
        Vector::ones(xs.len()).scalar_mul(self.params.get(0))
                              .add(&xs.scalar_mul(self.params.get(1)))
                              .unwrap()
    }

    fn cost_func(&self, xs: &Vector, ys: &Vector) -> f64 {
        let v = (self.model_func(xs).sub(ys)).unwrap();
        v.mul(&v).unwrap() / (2.0 * xs.len() as f64)
    }

    fn cost_gradient(&self, xs: &Vector, ys: &Vector) -> Vector<'r> {
        let n = xs.len();
        let v = (self.model_func(xs).sub(ys)).unwrap().scalar_mul(1.0 / n as f64);
        let d1 = (Vector::ones(n).mul(&v)).unwrap();
        let d2 = (xs.mul(&v)).unwrap();
        Vector::from_slice(&[d1, d2])
    }

    pub fn fit(&mut self, xs: &[f64], ys: &[f64]) {
        let xs_v = Vector::from_slice(xs);
        let ys_v = Vector::from_slice(ys);
        let mut cost_diff = 1.0f64;

        while cost_diff > self.tolerance {
            let priori = self.cost_func(&xs_v, &ys_v);
            let gradient = self.cost_gradient(&xs_v, &ys_v);
            self.params = (self.params.sub(&gradient.scalar_mul(self.step))).unwrap();
            let posteriori = self.cost_func(&xs_v, &ys_v);
            cost_diff = (priori - posteriori).abs();
        }
    }
}
