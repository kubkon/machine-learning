use std::fmt::{Display,Formatter,Result};
use Vector;
use core::optimisation::gradient_descent;

#[derive(Debug)]
pub struct LinearRegression<'r> {
    pub learning_rate: f64,
    pub tolerance: f64,
    pub params: Vector<'r>,
}

impl<'r> LinearRegression<'r> {
    pub fn new(learning_rate: f64, init_params: &Vector) -> LinearRegression<'r> {
        LinearRegression {
            learning_rate: learning_rate,
            tolerance: 1e-6,
            params: init_params.clone(),
        }
    }
    
    fn gradient_f(&self, xs: &Vector, ys: &Vector, params: &Vector) -> Vector<'r> {
        let n = xs.len();
        let v = Vector::ones(n).scalar_mul(params.get(0))
                               .add(&xs.scalar_mul(params.get(1)))
                               .sub(&ys)
                               .scalar_mul(1.0 / n as f64);
        let d1 = Vector::ones(n).mul(&v);
        let d2 = xs.mul(&v);
        Vector::from_slice(&[d1, d2])
    }

    pub fn fit(&mut self, xs: &Vector, ys: &Vector) {
        self.params = gradient_descent(
            self.learning_rate,
            self.tolerance,
            &self.params,
            |&: params: &Vector| -> Vector<'r> { self.gradient_f(xs, ys, params) }
        );
    }
}

impl<'r> Display for LinearRegression<'r> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "LinearRegression ( learning_rate={}, tolerance={}, parameters={} )",
               self.learning_rate,
               self.tolerance,
               self.params)
    }
}
