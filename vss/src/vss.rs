use bls12_381::{G1Projective, Scalar};
use ff::Field;
use rand::thread_rng;
use sss::{Polynomial, Share};

use crate::{Commitment, VssOutput};

/// Generate verifiable shares of a secret via the Feldman's verifiable secret sharing protocol.
/// Returns the shares and the polynomial coefficient commitment.
fn generate_vss_shares(secret: Scalar, t: usize, n: usize) -> VssOutput {
    let mut rng = thread_rng();

    let mut coefficients = Vec::with_capacity(t);
    coefficients.push(secret);
    for _ in 1..t {
        coefficients.push(Scalar::random(&mut rng));
    }
    let polynomial = Polynomial::new(coefficients.clone());

    let g = G1Projective::generator();
    let commitment_points: Vec<G1Projective> = coefficients.iter().map(|c| g * c).collect();
    let commitment = Commitment::new(commitment_points);

    let shares: Vec<Share> = (1..=n)
        .map(|i| {
            let x = Scalar::from(i as u64);
            let y = polynomial.evaluate(x);

            Share { x, y }
        })
        .collect();

    VssOutput { shares, commitment }
}

// Verify the share associated with one participant.
fn verify_share(share: &Share, commitment: &Commitment) -> bool {
    let g = G1Projective::generator();

    let lhs = g * share.y;
    let rhs = commitment.evaluate(share.x);

    lhs == rhs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vss_verification() {
        let mut rng = thread_rng();

        let t = 3;
        let n = 5;

        let secret = Scalar::random(&mut rng);

        let vss_data = generate_vss_shares(secret, t, n);

        for share in vss_data.shares {
            assert!(verify_share(&share, &vss_data.commitment));
        }
    }

    #[test]
    fn test_vss_tempered_commitment() {
        let mut rng = thread_rng();

        let t = 3;
        let n = 5;

        let secret = Scalar::random(&mut rng);

        let vss_data = generate_vss_shares(secret, t, n);

        let mut commitment = vss_data.commitment;
        commitment.points[0] = G1Projective::generator();

        assert!(!verify_share(&vss_data.shares[0], &commitment));
    }
}
