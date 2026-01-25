use std::collections::{HashMap, HashSet};

use bls12_381::{G1Projective, Scalar};
use ff::Field;
use rand::thread_rng;
use sss::{Polynomial, Share};
use vss::{verify_share, Commitment};

use crate::{DkgMessage, ParticipantId};

/// Return a vector of n participants associated with a t-secure threshold system.
pub fn create_n_participants_with_threshold(n: usize, t: usize) -> Vec<Participant> {
    (1..=n).map(|i| Participant::new(i, t, n)).collect()
}

/// Participant to the DKG protocol.
pub struct Participant {
    pub id: ParticipantId,

    // Protocol public data
    pub threshold: usize,
    pub num_participants: usize,
    pub group_public_key: Option<G1Projective>,

    // DKG data
    received_shares: HashMap<ParticipantId, Share>,
    received_commitments: HashMap<ParticipantId, Commitment>,
    pub qual_set: HashSet<ParticipantId>,

    // Signature info
    secret_polynomial: Option<Polynomial>,
    secret_share: Option<Scalar>,
}

impl Participant {
    pub fn new(id: ParticipantId, threshold: usize, num_participants: usize) -> Self {
        Self {
            id,
            threshold,
            num_participants,
            received_shares: HashMap::new(),
            received_commitments: HashMap::new(),
            qual_set: HashSet::new(),
            secret_polynomial: None,
            secret_share: None,
            group_public_key: None,
        }
    }

    /// Called on a participant acting as a dealer to generate the message to distribute shares to
    /// other participants.
    pub fn generate_shares(&mut self) -> Vec<DkgMessage> {
        let mut rng = thread_rng();

        // Generate random polynomial coefficients
        let mut coefficients = Vec::with_capacity(self.threshold);
        for _ in 0..self.threshold {
            coefficients.push(Scalar::random(&mut rng));
        }
        let polynomial = Polynomial::new(coefficients.clone());
        self.secret_polynomial = Some(polynomial.clone());

        // Create commitment (g^a_0, g^a_1, ..., g^a_{t-1})
        let g = G1Projective::generator();
        let commitment_points: Vec<G1Projective> = coefficients.iter().map(|c| g * c).collect();
        let commitment = Commitment::new(commitment_points);

        // Generate shares for each participant
        let shares: Vec<Share> = (1..=self.num_participants)
            .map(|i| {
                let x = Scalar::from(i as u64);
                let y = polynomial.evaluate(x);
                Share { x, y }
            })
            .collect();

        // Prepare messages
        let mut messages = Vec::new();
        messages.push(DkgMessage::BroadcastCommitment {
            from: self.id,
            commitment: commitment.clone(),
        });

        // Send each share to its corresponding participant
        for (i, share) in shares.iter().enumerate() {
            messages.push(DkgMessage::DistributeShare {
                from: self.id,
                to: i + 1, // Participant IDs start from 1
                share: share.clone(),
            });
        }

        messages
    }

    /// Verify tha the shares received are coherent with the dealer commitment.
    pub fn verify_shares(&mut self, messages: Vec<DkgMessage>) -> Vec<DkgMessage> {
        let mut complains = Vec::new();

        for msg in messages {
            match msg {
                DkgMessage::BroadcastCommitment { from, commitment } => {
                    self.received_commitments.insert(from, commitment);
                }
                DkgMessage::DistributeShare { from, to, share } => {
                    if self.id == to {
                        self.received_shares.insert(from, share.clone());
                    }
                }
                DkgMessage::BroadcastComplaint { .. } => todo!("This part is boring"),
            }
        }

        for (&sender_id, share) in &self.received_shares {
            if let Some(commitment) = self.received_commitments.get(&sender_id) {
                if verify_share(share, commitment) {
                    self.qual_set.insert(sender_id);
                } else {
                    complains.push(DkgMessage::BroadcastComplaint {
                        from: self.id,
                        against: sender_id,
                        reason: format!("Share verification failed"),
                    });
                }
            }
        }

        complains
    }

    /// Compute the group public key by combining all the received commitments and the participant
    /// private share, by multiplying all the received shares.
    pub fn compute_keys(&mut self) {
        let mut secret_share = Scalar::zero();
        let mut group_pk = G1Projective::identity();

        for (&sender_id, share) in &self.received_shares {
            if self.qual_set.contains(&sender_id) {
                secret_share += share.y;
                let sender_commitment = self.received_commitments.get(&sender_id).unwrap();
                group_pk += sender_commitment.points[0];
            }
        }
        self.secret_share = Some(secret_share);
        self.group_public_key = Some(group_pk)
    }

    /// Returns the delaer secret value used for the secret polynomial.
    pub fn get_secret_polynomial_intercept(&self) -> Option<Scalar> {
        if let Some(polynomnial) = &self.secret_polynomial {
            Some(polynomnial.evaluate(Scalar::zero()))
        } else {
            None
        }
    }

    /// Returns the participant secret share for the group private key.
    pub fn get_secret_share(&self) -> Option<Scalar> {
        self.secret_share
    }

    pub fn get_group_public_key(&self) -> Option<G1Projective> {
        self.group_public_key
    }
}
