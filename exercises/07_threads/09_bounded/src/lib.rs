// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: mpsc::SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, mpsc::TrySendError<()>> {
        let (response_sender, response_receiver) = mpsc::sync_channel(1);
        match self.sender.try_send(Command::Insert {
            draft,
            response_channel: response_sender,
        }) {
            Ok(_) => Ok(response_receiver.recv().unwrap()),
            Err(mpsc::TrySendError::Full(_)) => Err(mpsc::TrySendError::Full(())),
            Err(mpsc::TrySendError::Disconnected(_)) => Err(mpsc::TrySendError::Disconnected(())),
        }
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, mpsc::TrySendError<()>> {
        let (response_sender, response_receiver) = mpsc::sync_channel(1);
        match self.sender.try_send(Command::Get {
            id,
            response_channel: response_sender,
        }) {
            Ok(_) => Ok(response_receiver.recv().unwrap()),
            Err(mpsc::TrySendError::Full(_)) => Err(mpsc::TrySendError::Full(())),
            Err(mpsc::TrySendError::Disconnected(_)) => Err(mpsc::TrySendError::Disconnected(())),
        }
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: mpsc::SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: mpsc::SyncSender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
