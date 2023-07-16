use std::{
    fmt::{Display, Formatter, Result},
    sync::Arc,
};

use crate::types::*;

pub mod algorithm;
pub mod distance;
pub mod combo;

type Pod = Vec<Team>;
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PodTuple<const N: usize>([Arc<Pod>; N]);

impl<const N: usize> Display for PodTuple<N> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for i in 0..N {
            writeln!(f, "Pod {i}: {:?}", self.0[i])?;
        }
        Result::Ok(())
    }
}
