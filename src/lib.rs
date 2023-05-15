pub mod distance_lookup_table;
pub mod division_pairs;
pub mod types;

use crate::distance_lookup_table::create_lookup_table;
use crate::division_pairs::get_all_division_pairs;
use crate::types::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq)]
struct DivisionDistance {
    dist: u32,
    first: Division,
    second: Division,
}

impl PartialOrd<Self> for DivisionDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

impl Ord for DivisionDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

pub fn find_closest_divisions(conference: &Conference) {
    let len = conference.len() as u32;
    assert!(
        conference.len() >= 4,
        "The algorithm will fail for conference length < 4!"
    );
    let all_division_pairs = get_all_division_pairs(conference);
    let lookup_table = create_lookup_table();
    let distance = all_division_pairs
        .into_par_iter()
        .map(|(first, second)| {
            let first_sum = sum_division_dist(&first, &lookup_table);
            let second_sum = sum_division_dist(&second, &lookup_table);
            let dist = (first_sum + second_sum) / len;
            DivisionDistance {
                dist,
                first,
                second,
            }
        })
        .min()
        .expect("All division pairs are not empty!");
    assert!(!distance.first.is_empty() && !distance.second.is_empty());
    println!("Distance: {}", distance.dist);
    println!("First Division: {}", distance.first.join(", "));
    println!("Second Division: {}", distance.second.join(", "));
}

fn sum_division_dist(division: &Division, lookup_table: &HashMap<TeamPair, u32>) -> u32 {
    division
        .iter()
        .tuple_combinations::<(&&str, &&str)>()
        .map(|(a, b)| match lookup_table.get(&(a, b)) {
            Some(dist) => *dist,
            None => *lookup_table.get(&(b, a)).unwrap(),
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn sum_division_dist_in_random_order() {
        let mut conference = vec![
            "Brigham Young University",
            "Iowa State University",
            "Kansas State University",
            "Oklahoma State University",
            "Texas Christian University",
            "Texas Tech University",
            "University of Baylor",
            "University of Central Florida",
            "University of Cincinnati",
            "University of Houston",
            "University of Kansas",
            "University of Oklahoma",
            "University of Texas",
            "University of West Virginia",
        ];
        conference.shuffle(&mut thread_rng());

        let _ = find_closest_divisions(&conference);
    }
}
