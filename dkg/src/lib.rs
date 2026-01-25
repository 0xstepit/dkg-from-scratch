pub mod messages;
pub use messages::*;
pub mod network;
pub use network::*;
pub mod participants;
pub use participants::*;

#[cfg(test)]
mod tests {
    use crate::{DkgMessage, InMemoryNetwork, Network, create_n_participants_with_threshold};

    #[test]
    fn test_dkg_protocol() {
        let threshold = 3;
        let num_participants = 5;

        let mut network = InMemoryNetwork::new((1..=num_participants).collect());

        let mut participants = create_n_participants_with_threshold(num_participants, threshold);
        println!("\n### Participants and secrets ###");
        for participant in &mut participants {
            let messages = participant.generate_shares();
            println!(
                "Participant [{}] with polynomial secret [{}]",
                participant.id,
                participant.get_secret_polynomial_intercept().unwrap()
            );
            for msg in messages {
                match msg {
                    DkgMessage::DistributeShare { to, .. } => {
                        network.send_private(to, msg);
                    }
                    DkgMessage::BroadcastCommitment { .. } => {
                        network.broadcast(msg);
                    }
                    DkgMessage::BroadcastComplaint { .. } => {
                        todo!("We are all good people, we trust assume all honest participants")
                    }
                }
            }
        }

        for participant in &mut participants {
            let messages = network.receive(participant.id);
            let complaints = participant.verify_shares(messages);

            assert_eq!(complaints.len(), 0, "Expected all honest participants")
        }

        println!("\n### QUAL set used ###");
        for participant in &participants {
            println!(
                "Participant [{}] qual_set: [{:?}]",
                participant.id, participant.qual_set
            );
        }

        for participant in &mut participants {
            participant.compute_keys();
        }

        println!("\n### Final keys ###");
        let first_pk = participants[0].get_group_public_key().unwrap();
        for participant in &participants {
            let pk = participant.get_group_public_key().unwrap();
            assert_eq!(pk, first_pk, "Recovered public keys are different");
            println!(
                "Participant [{}] has public key [{}]",
                participant.id, first_pk
            );
        }

        for participant in &participants {
            assert_eq!(
                participant.get_group_public_key().unwrap(),
                first_pk,
                "Participant [{}] has different group public key!",
                participant.id
            );
        }
    }
}
