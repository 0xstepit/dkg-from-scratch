use crate::messages::{DkgMessage, ParticipantId};
use std::collections::{HashMap, VecDeque};

pub trait Network {
    // Send a private message to a network participant.
    fn send_private(&mut self, to: ParticipantId, msg: DkgMessage);

    // Broadcast a message to all network participant.
    fn broadcast(&mut self, msg: DkgMessage);

    // Accumulate all the messages received in the mailbox.
    fn receive(&mut self, id: ParticipantId) -> Vec<DkgMessage>;
}

/// Simplified network used to locally test DKG.
pub struct InMemoryNetwork {
    inboxes: HashMap<ParticipantId, VecDeque<DkgMessage>>,
}

impl InMemoryNetwork {
    pub fn new(participant_ids: Vec<ParticipantId>) -> Self {
        let mut inboxes = HashMap::new();
        for participant_id in participant_ids {
            inboxes.insert(participant_id, VecDeque::new());
        }

        Self { inboxes }
    }
}

impl Network for InMemoryNetwork {
    fn send_private(&mut self, to: ParticipantId, msg: DkgMessage) {
        if let Some(inbox) = self.inboxes.get_mut(&to) {
            inbox.push_back(msg);
        }
    }

    fn broadcast(&mut self, msg: DkgMessage) {
        for (_, inbox) in self.inboxes.iter_mut() {
            inbox.push_back(msg.clone());
        }
    }

    fn receive(&mut self, id: ParticipantId) -> Vec<DkgMessage> {
        if let Some(inbox) = self.inboxes.get_mut(&id) {
            inbox.drain(..).collect()
        } else {
            Vec::new()
        }
    }
}
