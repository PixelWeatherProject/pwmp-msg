//! This module contains types related to diagnostic artifacts.

use serde::{Deserialize, Serialize};

/// Stores the contents of a diagnostic artifact.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Artifact {
    /// UTF-8 encoded string.
    String(Box<str>),

    /// A blob of bytes.
    #[serde(with = "serde_bytes")]
    Blob(Box<[u8]>),
}

impl Artifact {
    /// Returns whether the artifact contains no data.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        match self {
            Self::Blob(b) => b.is_empty(),
            Self::String(s) => s.is_empty(),
        }
    }

    /// Returns the length of the data held by the artifact.
    #[must_use]
    pub const fn len(&self) -> usize {
        match self {
            Self::Blob(b) => b.len(),
            Self::String(s) => s.len(),
        }
    }
}

impl std::fmt::Debug for Artifact {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(value) if value.len() <= 32 => {
                formatter.debug_tuple("String").field(value).finish()
            }
            Self::String(value) => write!(formatter, "String([{} chars])", value.len()),
            Self::Blob(value) => write!(formatter, "Blob([{} bytes])", value.len()),
        }
    }
}

impl From<Box<[u8]>> for Artifact {
    fn from(value: Box<[u8]>) -> Self {
        Self::Blob(value)
    }
}

impl From<String> for Artifact {
    fn from(value: String) -> Self {
        Self::String(value.into_boxed_str())
    }
}

impl From<Box<str>> for Artifact {
    fn from(value: Box<str>) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Artifact {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<&[u8]> for Artifact {
    fn from(value: &[u8]) -> Self {
        Self::Blob(value.into())
    }
}

impl<const N: usize> From<&[u8; N]> for Artifact {
    fn from(value: &[u8; N]) -> Self {
        Self::from(&value[..])
    }
}

#[cfg(test)]
mod _tests {
    use super::Artifact;

    #[test]
    fn debug_print() {
        // short string
        assert_eq!(
            format!("{:?}", Artifact::from("Hello, World!")),
            String::from("String(\"Hello, World!\")")
        );

        // long string
        assert_eq!(
            format!("{:?}", Artifact::from("Hello, World! This is now longer!")),
            String::from("String([33 chars])")
        );

        // blob
        assert_eq!(
            format!("{:?}", Artifact::from(&[0xFF, 0xAA])),
            String::from("Blob([2 bytes])")
        );
    }
}
