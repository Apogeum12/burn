#![feature(generic_associated_types)]

#[macro_use]
extern crate derive_new;

pub(crate) mod graph;
pub use graph::grad::Gradients;

mod tensor;
pub use tensor::*;

pub use half::f16;
