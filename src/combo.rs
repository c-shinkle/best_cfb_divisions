use crate::{Conference, Division};
use std::collections::HashMap;

use itertools::Itertools;

// time to beat!
// 219,513 ns/iter (+/- 4,968)
// best so far
// 230,544 ns/iter (+/- 2,306)
pub fn get_all_division_pairs(conference: &Conference) -> Vec<(Division, Division)> {
    let len = conference.len() as u32;
    let list_of_indices = (0..len)
        .combinations(conference.len() / 2)
        .map(|indices| indices.to_vec())
        .collect::<Vec<Vec<u32>>>();

    let mut map = HashMap::<Vec<u32>, (Division, Division)>::with_capacity(list_of_indices.len() / 2);
    for indices in list_of_indices {
        if !map.contains_key(&indices) {
            let complement = create_complement(len, &indices);
            if !map.contains_key(&complement) {
                let first = indices
                    .iter()
                    .map(|i| conference[*i as usize])
                    .collect::<Division>();
                let second = complement
                    .iter()
                    .map(|i| conference[*i as usize])
                    .collect::<Division>();
                map.insert(indices, (first, second));
            }
        }
    }

    map.into_values().collect()
}

fn create_complement(len: u32, indices: &[u32]) -> Vec<u32> {
    (0..len)
        .into_iter()
        .filter(|i| !indices.contains(i))
        .collect()
}
