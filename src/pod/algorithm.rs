use std::collections::HashMap;

use itertools::Itertools;

use super::quadruple::get_all_pod_quadruples;
use crate::distance_lookup_table::create_lookup_table;
use crate::pod::distance::PodDistance;
use crate::types::*;

pub fn find_closest_pods(conference: &Conference) {
    assert!(
        conference.len() >= 4,
        "The algorithm will fail for conference length < 4!"
    );
    assert!(
        conference.len() % 4 == 0,
        "The algorithm will fail for conference length not divisionable by 4!"
    );
    let all_pod_quadruples = get_all_pod_quadruples(conference);
    for pod_quad in all_pod_quadruples {
        println!("{:?}", pod_quad);
    }
    // let lookup_table = create_lookup_table();
    // let distance = all_pod_quadruples
    //     .into_iter()
    //     .map(|pods| {
    //         let distance = (sum_pod_dist(&pods.0, &lookup_table)
    //             + sum_pod_dist(&pods.1, &lookup_table)
    //             + sum_pod_dist(&pods.2, &lookup_table)
    //             + sum_pod_dist(&pods.3, &lookup_table))
    //             / 4;
    //         PodDistance::new(distance, pods)
    //     })
    //     .min()
    //     .expect("All division pairs are not empty!");
    // println!("{distance}");
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
