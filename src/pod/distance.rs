use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};

use super::PodTuple;

#[derive(Clone, Eq, PartialEq)]
pub struct PodDistance<const N: usize> {
    distance: u32,
    pods: PodTuple<N>,
}

impl<const N: usize> PodDistance<N> {
    pub fn new(distance: u32, pods: PodTuple<N>) -> PodDistance<N> {
        PodDistance { distance, pods }
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
            writeln!(f, "Pod {i}: {:?}", self.pods.0[i])?;
        }
        Result::Ok(())
    }
}
