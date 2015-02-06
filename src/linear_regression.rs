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

    fn model_func(self, xs: Vector) -> Vector {
        (Vector::ones(xs.len()).scalar_mul(*self.params.get(0)) +
         xs.scalar_mul(*self.params.get(1))).unwrap()
    }

    // pub fn fit(&mut self, xs: &[f64], ys: &[f64]) {
    //     let m = xs.len() as f64;
    //     let mut cost_diff = 1.0f64;

    //     while cost_diff > self.tolerance {
    //         cost_diff = xs.iter()
    //                       .zip(ys.iter())
    //                       .map(|(&x, &y)| (self.params[0] + self.params[1]*x - y).powi(2))
    //                       .fold(0.0, |acc, x| acc + x) / (2.0 * m);
    //         let d1 = xs.iter()
    //                    .zip(ys.iter())
    //                    .map(|(&x, &y)| self.params[0] + self.params[1]*x - y)
    //                    .fold(0.0, |acc, x| acc + x) / m;
    //         let d2 = xs.iter()
    //                    .zip(ys.iter())
    //                    .map(|(&x, &y)| x * (self.params[0] + self.params[1]*x - y))
    //                    .fold(0.0, |acc, x| acc + x) / m;
    //         self.params[0] = self.params[0] - self.step * d1;
    //         self.params[1] = self.params[1] - self.step * d2;
    //         let cost = xs.iter()
    //                      .zip(ys.iter())
    //                      .map(|(&x, &y)| (self.params[0] + self.params[1]*x - y).powi(2))
    //                      .fold(0.0, |acc, x| acc + x) / (2.0 * m);
    //         cost_diff = (cost_diff - cost).abs();
    //     }
    // }
}
