use crate::{Conference, Division};
use itertools::Itertools;
use std::collections::HashMap;

pub fn get_all_division_pairs(conference: &Conference) -> Vec<(Division, Division)> {
    let len = conference.len() as u32;
    let list_of_indices = (0..len)
        .combinations(conference.len() / 2)
        .map(|indices| indices.to_vec())
        .collect::<Vec<Vec<u32>>>();

    let mut map =
        HashMap::<Vec<u32>, (Division, Division)>::with_capacity(list_of_indices.len() / 2);
    for indices in list_of_indices {
        if !map.contains_key(&indices) {
            let complement = (0..len)
                .into_iter()
                .filter(|i| !indices.contains(i))
                .collect::<Vec<u32>>();
            if !map.contains_key(&complement) {
                let first = indices
                    .iter()
                    .map(|i| conference[*i as usize])
                    .collect::<Division>();
                let second = complement
                    .into_iter()
                    .map(|i| conference[i as usize])
                    .collect::<Division>();
                map.insert(indices, (first, second));
            }
        }
    }

    map.into_values().collect()
}
