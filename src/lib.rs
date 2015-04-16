#![crate_name = "machinelearning"]

pub use core::vector::Vector;
pub use linear_regression::LinearRegression;

mod core;
mod linear_regression;

#[macro_use]
mod macros;

#[cfg(test)]
mod test;
