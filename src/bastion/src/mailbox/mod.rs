pub mod envelope;
pub mod message;

pub(crate) mod mailbox;
pub(crate) mod state;

pub use crate::mailbox::mailbox::Mailbox;
pub use crate::mailbox::message::{Message, MessageType};