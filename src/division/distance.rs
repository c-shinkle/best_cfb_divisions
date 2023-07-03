use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};

use crate::types::Division;

#[derive(Clone, Eq, PartialEq)]
pub struct DivisionDistance {
    distance: u32,
    first: Division,
    second: Division,
}

impl DivisionDistance {
    pub fn new(distance: u32, first: Division, second: Division) -> DivisionDistance {
        DivisionDistance {
            distance,
            first,
            second,
        }
    }
}

impl PartialOrd for DivisionDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for DivisionDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl Display for DivisionDistance {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "Distance: {}", self.distance)?;
        writeln!(f, "First Division: {}", self.first.join(", "))?;
        writeln!(f, "Second Division: {}", self.second.join(", "))?;
        Result::Ok(())
    }
}
