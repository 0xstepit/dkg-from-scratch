//! Library implementing the Samir's Secret Sharing protocol to distribute
//! a secret between `n` parties. The degree of the polynomial used in the
//! secret distribution defines the threshold on the number of shares required
//! to reconstruct the secret.
pub mod sss;
pub use sss::*;

use bls12_381::Scalar;

/// A share of a secret given by the point evaluation of a polynomial.
#[derive(Clone)]
pub struct Share {
    /// Public information given by the index of the share recipient.
    pub x: Scalar,
    /// Secret value of the share.
    pub y: Scalar,
}

/// A minimla implementation of a univariate polynomial.
#[derive(Clone)]
pub struct Polynomial {
    /// Random coefficients.
    coeffs: Vec<Scalar>,
}

impl Polynomial {
    pub fn new(coeffs: Vec<Scalar>) -> Self {
        Self { coeffs }
    }

    // Evaluate the polynomial using the Horner's method.
    pub fn evaluate(&self, x: Scalar) -> Scalar {
        let mut y = Scalar::zero();
        for coeff in self.coeffs.iter().rev() {
            y = (y * x) + *coeff;
        }

        y
    }
}
