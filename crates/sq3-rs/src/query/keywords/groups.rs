use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Groups;
impl Groups {
    pub const fn as_str() -> &'static str {
        "GROUPS"
    }
}

impl PartialEq<&str> for Groups {
    fn eq(&self, other: &&str) -> bool {
        Groups::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Groups> for &str {
    fn eq(&self, _: &Groups) -> bool {
        Groups::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Groups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Groups {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
