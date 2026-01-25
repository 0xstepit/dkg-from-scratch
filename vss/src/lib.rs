pub mod vss;
pub use vss::*;

use bls12_381::{G1Projective, Scalar};
use sss::Share;

/// Output of the Feldman's verifiable secret sharing protocol.
pub struct VssOutput {
    /// Shares to distribute to protocol participants.
    pub shares: Vec<Share>,
    /// Public commitment for the random polynomial coefficients.
    pub commitment: Commitment,
}

/// Polynomial commitments on the group projective space.
#[derive(Clone)]
pub struct Commitment {
    pub points: Vec<G1Projective>,
}

impl Commitment {
    pub fn new(points: Vec<G1Projective>) -> Self {
        Self { points }
    }

    // Evaluate the polynomial using the Horner's method in the generator exponent.
    pub fn evaluate(&self, x: Scalar) -> G1Projective {
        let mut y = G1Projective::identity();
        for point in self.points.iter().rev() {
            y = (y * x) + *point;
        }

        y
    }
}
