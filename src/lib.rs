#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![cfg_attr(feature = "nightly", feature(test))]

//! A Tokio codec implementation of the WebSocket protocol.
//!
//! This crate does not do any I/O directly. For a full WebSocket client, see the [websocket-lite](https://docs.rs/websocket-lite) crate.

mod frame;
mod mask;
mod message;
mod opcode;
mod upgrade;

pub mod protocol;

pub use crate::message::{Message, MessageCodec};
pub use crate::opcode::Opcode;
pub use crate::upgrade::{ClientRequest, UpgradeCodec};

use std::error;
use std::result;

/// Represents errors that can be exposed by this crate.
pub type Error = Box<dyn error::Error + Send + Sync + 'static>;

/// Represents results returned by the non-async functions in this crate.
pub type Result<T> = result::Result<T, Error>;