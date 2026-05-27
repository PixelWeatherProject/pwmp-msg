//! This module contains a schema version identifier.
//!
//! The schema version allows the server implementation to check if the client is using a supported schema version.
//! The server may reject connections, if the client is using an outdated/incompatible schema.

use serde::{Deserialize, Serialize};

/// Message schema version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum SchemaVersion {
    /// Schema version as of release 3.0.0.
    V3_0_0,
}
