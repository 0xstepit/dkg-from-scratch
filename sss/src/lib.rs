use bls12_381::Scalar;

mod sss;

struct Share {
    x: Scalar,
    y: Scalar,
}

struct Polynomial {
    coeffs: Vec<Scalar>,
}

impl Polynomial {
    pub fn new(coeffs: Vec<Scalar>) -> Self {
        Self { coeffs: coeffs }
    }

    // Evaluate the polynomial using the Horner's method
    pub fn evaluate(&self, x: Scalar) -> Scalar {
        let mut y = Scalar::zero();

        for coeff in self.coeffs.iter().rev() {
            y = (y * x) + *coeff;
        }

        y
    }
}
