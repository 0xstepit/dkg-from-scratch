use crate::{Polynomial, Share};
use bls12_381::Scalar;
use ff::Field;
use rand::thread_rng;

/// Generates shares for the secret using a (t, n) threshold scheme. `t` provides the degree of the
/// polynomial which intercept value is the distributed secret, and `n` is the total number
/// of shares to create.
pub fn generate_shares(secret: Scalar, t: usize, n: usize) -> Vec<Share> {
    // Add the secret as the zero coefficient.
    let mut coefficients = Vec::with_capacity(t);
    coefficients.push(secret);

    // Generate random numbers create the poly's coefficients. The resulting polynomial will
    // be of degree t-1.
    let mut rng = thread_rng();
    for _ in 1..t {
        coefficients.push(Scalar::random(&mut rng));
    }
    let polynomial = Polynomial::new(coefficients);

    // Generate the shares for each participant.
    let shares: Vec<Share> = (1..=n)
        .map(|i| {
            let x = Scalar::from(i as u64);
            let y = polynomial.evaluate(x);

            Share { x, y }
        })
        .collect();

    shares
}

// Recover a secret by interpolating a polynomial from the provided shares.
pub fn reconstruct_secret(shares: &[Share]) -> Scalar {
    // Use Lagrange interpolation in 0 to reconstruct the secret
    let mut s = Scalar::zero();
    for i in 0..shares.len() {
        let y = shares[i].y;
        let x = shares[i].x;

        let mut num = Scalar::one();
        let mut den = Scalar::one();

        for j in 0..shares.len() {
            if j == i {
                continue;
            }
            num *= shares[j].x;
            den *= shares[j].x - x;
        }

        // Since we are working with finite fields, the division is implemented as the
        // multiplication for the inverse of the denominator.
        let lagrange_coeff = num * den.invert().unwrap();
        s += lagrange_coeff * y;
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shamir_reconstruction() {
        let mut rng = thread_rng();
        // SSS threshold (3, 5)
        let t = 3;
        let n = 5;

        let secret = Scalar::random(&mut rng);
        let shares = generate_shares(secret, t, n);

        // Recover the secret using a subset of participants of order t.
        let subset = &shares[0..t];
        let recovered_secret = reconstruct_secret(subset);

        assert_eq!(secret, recovered_secret, "Reconstruction failed!")
    }

    #[test]
    fn test_shamir_reconstruction_different_subsets() {
        let mut rng = thread_rng();
        // SSS threshold (3, 10)
        let t = 3;
        let n = 10;

        let secret = Scalar::random(&mut rng);
        let shares = generate_shares(secret, t, n);

        for i in 0..n - t {
            let subset = &shares[i..i + t];
            let recovered_secret = reconstruct_secret(subset);

            assert_eq!(
                secret,
                recovered_secret,
                "Reconstruction failed with participants from {start} to {end}!",
                start = i,
                end = i + t
            );
        }
    }

    #[test]
    fn test_shamir_reconstruction_failure() {
        let mut rng = thread_rng();
        // SSS threshold (3, 5)
        let t = 3;
        let n = 5;

        let secret = Scalar::random(&mut rng);
        let shares = generate_shares(secret, t, n);

        let subset = &shares[0..t - 1];
        let recovered_secret = reconstruct_secret(subset);

        assert_ne!(
            secret, recovered_secret,
            "Reconstruction should fail with less shares than threshold!"
        )
    }
}
