use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};

use crate::types::Division;

#[derive(Clone, Eq, PartialEq)]
pub struct DivisionDistance {
    dist: u32,
    first: Division,
    second: Division,
}

impl DivisionDistance {
    pub fn new(dist: u32, first: Division, second: Division) -> DivisionDistance {
        DivisionDistance {
            dist,
            first,
            second,
        }
    }
}

impl PartialOrd<Self> for DivisionDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

impl Ord for DivisionDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl Display for DivisionDistance {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "Distance: {}", self.dist)?;
        writeln!(f, "First Division: {}", self.first.join(", "))?;
        writeln!(f, "Second Division: {}", self.second.join(", "))?;
        Result::Ok(())
    }
}
