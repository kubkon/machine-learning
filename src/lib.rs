#![crate_name = "machinelearning"]
#![feature(core)]
#![feature(collections)]

pub use core::vector::Vector;
pub use linear_regression::LinearRegression;

mod core;
mod linear_regression;

#[macro_use]
mod macros;

#[cfg(test)]
mod test;
