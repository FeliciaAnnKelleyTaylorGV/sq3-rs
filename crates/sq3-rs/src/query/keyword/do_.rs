use std::{any::Any, fmt::Display};

use crate::query::traits::SqliteKeyword;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Do;
impl Do {
    pub const fn as_str() -> &'static str {
        "DO"
    }
}

impl PartialEq<&str> for Do {
    fn eq(&self, other: &&str) -> bool {
        Do::as_str().eq_ignore_ascii_case(other)
    }
}

impl PartialEq<Do> for &str {
    fn eq(&self, _: &Do) -> bool {
        Do::as_str().eq_ignore_ascii_case(self)
    }
}

impl Display for Do {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

impl SqliteKeyword for Do {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }
}
