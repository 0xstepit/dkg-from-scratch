use sss::Share;
use vss::Commitment;

/// Identifier for the participant in the network.
pub type ParticipantId = usize;

#[derive(Clone)]
/// Possible messages exchanged in the protocol.
pub enum DkgMessage {
    /// Message sent from the dealer to all other players, in private, to distribute the dealer secret.
    DistributeShare {
        from: ParticipantId,
        to: ParticipantId,
        share: Share,
    },
    /// Message sent from the delaer publicly to commit to the polynomial used in the share
    /// dealing.
    BroadcastCommitment {
        from: ParticipantId,
        commitment: Commitment,
    },
    /// Message sent by one of the participant that received a share not coherent with the public
    /// commitment.
    BroadcastComplaint {
        from: ParticipantId,
        against: ParticipantId,
        reason: String,
    },
}
