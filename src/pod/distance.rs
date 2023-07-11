use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};

use crate::types::*;

#[derive(Clone, Eq, PartialEq)]
pub struct PodDistance {
    distance: u32,
    pods: PodQuadruple,
}

impl PodDistance {
    pub fn new(distance: u32, pods: PodQuadruple) -> PodDistance {
        PodDistance { distance, pods }
    }
}

impl PartialOrd for PodDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for PodDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl Display for PodDistance {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "Distance: {}", self.distance)?;
        writeln!(f, "First Pod: {:?}", self.pods.0)?;
        writeln!(f, "Second Pod: {:?}", self.pods.1)?;
        writeln!(f, "Third Pod: {:?}", self.pods.2)?;
        writeln!(f, "Fourth Pod: {:?}", self.pods.3)?;
        Result::Ok(())
    }
}
