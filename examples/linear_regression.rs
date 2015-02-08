extern crate machinelearning;

use machinelearning::{LinearRegression,Vector};

fn main() {
    let xs = [Vector::from_slice(&[1.0]),
              Vector::from_slice(&[2.0]),
              Vector::from_slice(&[4.0]),
              Vector::from_slice(&[0.0])];
    let ys = Vector::from_slice(&[0.5,1.0,2.0,0.0]);
    let n = xs[0].len() + 1;
    let mut model = LinearRegression::new(0.1, &Vector::zeros(n));
    model.tolerance = 1e-8;
    model.fit(&xs, &ys);
    println!("{:?}", model);
}
