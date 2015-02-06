#![crate_name = "machinelearning"]
#![feature(core)]
#![feature(collections)]

pub mod vector;
pub mod linear_regression;

#[macro_use]
mod macros;

#[cfg(test)]
mod test;
