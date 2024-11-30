use serde::{Deserialize, Serialize};

/// A type that allows containing multiple data types.
/// Used to represent node setting values.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettingValue {
    Number(u16),
    Boolean(bool),
}

macro_rules! impl_simple_from {
    ($src: ty, $variant: ident) => {
        impl From<$src> for SettingValue {
            fn from(value: $src) -> Self {
                Self::$variant(value)
            }
        }
    };
}

macro_rules! impl_simple_getter {
    ($name: ident, $t: ty, $variant: ident) => {
        #[must_use]
        pub const fn $name(self) -> Option<$t> {
            if let Self::$variant(value) = self {
                Some(value)
            } else {
                None
            }
        }
    };
}

impl_simple_from!(u16, Number);
impl_simple_from!(bool, Boolean);

impl SettingValue {
    impl_simple_getter!(as_number, u16, Number);
    impl_simple_getter!(as_bool, bool, Boolean);
}
