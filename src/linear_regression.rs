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
    
    fn gradient_f(&self, xs: &[Vector], ys: &Vector, params: &Vector) -> Vector<'r> {
        let m = xs.len() as f64;
        let n = params.len();
        xs.iter()
          .zip(ys.iter())
          .map(|(x,&y)| x.scalar_mul((x.mul(&params) - y) / m))
          .fold(Vector::zeros(n), |acc, x| acc.add(&x))
    }

    fn extend_feature_vector(&self, xs: &[Vector]) -> Vec<Vector<'r>> {
        let mut xs_ext = Vec::new();
        for x in xs.iter() {
            let mut tmp = x.clone();
            tmp.insert(0, 1.0);
            xs_ext.push(tmp);
        }
        xs_ext
    }

    pub fn fit(&mut self, xs: &[Vector], ys: &Vector) {
        let xs_ext = self.extend_feature_vector(xs);
        self.params = gradient_descent(
            self.learning_rate,
            self.tolerance,
            &self.params,
            |&: params: &Vector| -> Vector<'r> { self.gradient_f(xs_ext.as_slice(), ys, params) }
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
