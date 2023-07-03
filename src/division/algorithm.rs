use super::distance::DivisionDistance;
use super::pair::division_pair_set::get_all_division_pairs;
use crate::distance_lookup_table::create_lookup_table;
use crate::types::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

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
            DivisionDistance::new((first_sum + second_sum) / len, first, second)
        })
        .min()
        .expect("All division pairs are not empty!");
    println!("{distance}");
}

fn sum_division_dist(division: &Division, lookup_table: &HashMap<TeamPair, u32>) -> u32 {
    division
        .iter()
        .tuple_combinations::<(&&str, &&str)>()
        .map(|(a, b)| {
            let first = lookup_table.get(&(a, b));
            first.or_else(|| lookup_table.get(&(b, a))).unwrap()
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
            "Indiana University",
            "Michigan State University",
            "Northwestern University",
            "Ohio State University",
        ];
        conference.shuffle(&mut thread_rng());

        let _ = find_closest_divisions(&conference);
    }
}
