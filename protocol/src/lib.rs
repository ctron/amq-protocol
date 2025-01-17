#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/amq-protocol/2.0.0-rc16/")]

//! # AMQP manipulation library
//!
//! amq-protocol is a library aiming at providing tools to help
//! implementing software using AMQP

/// AMQP Frame handling utils
pub mod frame;
/// The AMQ Protocol implementation (Generated)
pub mod protocol;
/// Utility to handle sasl auth with AMQP server
pub mod sasl;
/// Reexport of amq_protocol_tcp
pub mod tcp;
/// Reexport of amq_protocol_types
pub mod types;
/// Reexport of amq_protocol_uri
pub mod uri;
