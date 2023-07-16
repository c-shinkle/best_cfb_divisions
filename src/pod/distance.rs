use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};

use super::combo::PodCombo;

#[derive(Clone, Eq, PartialEq)]
pub struct PodDistance<const N: usize> {
    distance: u32,
    pod_combo: PodCombo<N>,
}

impl<const N: usize> PodDistance<N> {
    pub fn new(distance: u32, pod_combo: PodCombo<N>) -> PodDistance<N> {
        PodDistance {
            distance,
            pod_combo,
        }
    }
}

impl<const N: usize> PartialOrd for PodDistance<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl<const N: usize> Ord for PodDistance<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl<const N: usize> Display for PodDistance<N> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "Distance: {}", self.distance)?;
        for i in 0..N {
            writeln!(f, "Pod {i}: {:?}", self.pod_combo[i])?;
        }
        Result::Ok(())
    }
}
