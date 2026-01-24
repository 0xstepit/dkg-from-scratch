use bls12_381::{G1Projective, Scalar};
use group::Group;
use sss::Share;

mod vss;

pub struct VssData {
    pub shares: Vec<Share>,
    pub commitment: Commitment,
}

pub struct Commitment {
    pub points: Vec<G1Projective>,
}

impl Commitment {
    pub fn new(points: Vec<G1Projective>) -> Self {
        Self { points: points }
    }

    pub fn evaluate(&self, x: Scalar) -> G1Projective {
        let mut y = G1Projective::identity();

        for point in self.points.iter().rev() {
            y = (y * x) + *point;
        }

        y
    }
}
