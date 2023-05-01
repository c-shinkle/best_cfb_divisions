use crate::{Conference, DivisionPair};
use itertools::Itertools;
use std::collections::HashMap;

pub fn get_all_division_pairs(conference: &Conference) -> Vec<DivisionPair> {
    let len = conference.len();
    let index_combinations = (0..len).combinations(len / 2).collect::<Vec<Vec<usize>>>();

    let mut map = HashMap::with_capacity(index_combinations.len() / 2);
    let index_into = |i: usize| conference[i];
    for combo in index_combinations {
        if !map.contains_key(&combo) {
            let complement = (0..len)
                .into_iter()
                .filter(|i| !combo.contains(i))
                .collect::<Vec<usize>>();
            if !map.contains_key(&complement) {
                let division_pair = (
                    combo.iter().copied().map(index_into).collect(),
                    complement.into_iter().map(index_into).collect(),
                );
                map.insert(combo, division_pair);
            }
        }
    }

    map.into_values().collect()
}
