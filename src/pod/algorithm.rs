use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

use super::combo::get_all_pod_combinations;
use crate::distance_lookup_table::create_lookup_table;
use crate::pod::combo::Pod;
use crate::pod::distance::PodDistance;
use crate::types::*;

pub fn find_closest_pods<const POD_COUNT: usize>(conference: &Conference) {
    assert!(
        conference.len() / POD_COUNT == 4,
        "Conference length divided by Pod coutn must equal 4!"
    );
    let all_pod_quadruples = get_all_pod_combinations::<POD_COUNT>(conference);
    let lookup_table = create_lookup_table();
    let distance = all_pod_quadruples
        .into_par_iter()
        .map(|pods| {
            let distance = (0..POD_COUNT)
                .map(|i| sum_pod_dist(&pods[i], &lookup_table))
                .sum::<u32>()
                / POD_COUNT as u32;
            PodDistance::new(distance, pods)
        })
        .min()
        .expect("All pod quadruples are not empty!");
    println!("{distance}");
}

fn sum_pod_dist(pod: &Pod, lookup_table: &HashMap<TeamPair, u32>) -> u32 {
    pod.iter()
        .tuple_combinations()
        .map(|(a, b): (&&str, &&str)| {
            let first = lookup_table.get(&(a, b));
            first.or_else(|| lookup_table.get(&(b, a))).unwrap()
        })
        .sum()
}
