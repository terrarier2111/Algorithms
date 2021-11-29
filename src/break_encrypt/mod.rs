use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub mod caesar;

pub struct BreakError {
    msg: String,
}

impl Debug for BreakError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg.as_str())
    }
}

impl Display for BreakError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg.as_str())
    }
}

impl Error for BreakError {
}