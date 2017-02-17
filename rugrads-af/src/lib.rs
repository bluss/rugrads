extern crate rugrads;
extern crate arrayfire as libaf;

mod arrayfire;

#[cfg(test)]
pub mod testsupport;

use libaf::DType;
use rugrads::Expression;

// Reexport specialized rugrad types
pub type Container<E> = rugrads::Container<Array, E>;
pub type Context = rugrads::Context<Array>;
pub struct Gradient<E: Expression<Array>>(pub rugrads::Gradient<Array, E>);

// Reexport some needed libaf api components
pub use libaf::{Array, Backend, NormType, MatProp, Dim4};
pub use libaf::{set_backend, constant};

// Reexport all arrayfire wrapper functions
pub use arrayfire::wrappers::*;

/// A struct for two dimensions
pub struct Dim2(pub [u64; 2]);

impl Into<libaf::Dim4> for Dim2 {
    fn into(self) -> libaf::Dim4 {
        libaf::Dim4::new(&[self.0[0], self.0[1], 1, 1])
    }
}

/// Creates a new 2d Array
pub fn new_array(slice: &[f64], dims: Dim2) -> Array {
    Array::new(slice, dims.into())
}

impl<E: Expression<Array>> Gradient<E> {
    pub fn of(expr: Container<E>, context: Context) -> Self {
        Gradient(rugrads::Gradient::of(expr, context))
    }

    pub fn grad(&mut self, wrt: Container<rugrads::Variable>) -> Array {
        let output_dims = wrt.inner().value(&self.0.context()).dims();
        let output_type = wrt.inner().value(&self.0.context()).get_type();
        let output_elms = output_dims.elements() as usize;

        match output_type {
            DType::F64 => {
                let ones = vec![1f64; 4];
                self.0.backprop(wrt, Array::new(&ones, Dim4::new(&[4,1,1,1])))
            },
            _ => panic!("Currently only f64 array types are supported")
        }
    }
}
