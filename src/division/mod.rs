use crate::types::Team;

pub mod algorithm;
mod distance;
mod pair;

type Division = Vec<Team>;
type DivisionPair = (Division, Division);
