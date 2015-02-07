extern crate machinelearning;

use machinelearning::vector::Vector;
use machinelearning::linear_regression::LinearRegression;

fn main() {
    let xs = Vector::from_slice(&[1.0,2.0,4.0,0.0]);
    let ys = Vector::from_slice(&[0.5,1.0,2.0,0.0]);
    let mut model = LinearRegression::new(0.1, 1e-6);
    model.fit(&xs, &ys);
    println!("{:?}", model);
}
