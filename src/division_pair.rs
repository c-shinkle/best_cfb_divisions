use crate::types::{Division, DivisionPair};
use crate::Conference;
use itertools::Itertools;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Pair {
    first: Vec<usize>,
    second: Vec<usize>,
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first
    }
}

impl Eq for Pair {}

impl Hash for Pair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.first.hash(state);
    }
}

pub fn get_all_division_pairs(conference: &Conference) -> Vec<DivisionPair> {
    let len = conference.len();
    let index_combinations: Vec<Vec<usize>> = (0..len).combinations(len / 2).collect();
    let division_pairs_len = index_combinations.len() / 2;

    let mut set: HashSet<Pair> = HashSet::with_capacity(division_pairs_len);
    let index_into = |i: usize| conference[i];

    for combo in index_combinations {
        let comp: Vec<usize> = (0..len).filter(|i| !combo.contains(i)).collect();
        let original_pair = Pair {
            first: combo.clone(),
            second: comp.clone(),
        };
        let comp_pair = Pair {
            first: comp,
            second: combo,
        };
        if !set.contains(&comp_pair) {
            println!("Inserting {original_pair:?}");
            set.insert(original_pair);
        }
    }

    set.into_iter()
        .map(|Pair { first, second }| {
            let a = first.into_iter().map(index_into).collect::<Division>();
            let b = second.into_iter().map(index_into).collect::<Division>();
            (a, b)
        })
        .collect()
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::types::DivisionPair;

    use super::get_all_division_pairs;

    #[test]
    fn should_get_no_symmetric_division() {
        let conference = ["A", "B", "C", "D"];
        let actual: Vec<DivisionPair> = get_all_division_pairs(&conference);

        let expected = HashSet::from([
            (vec!["A", "B"], vec!["C", "D"]),
            (vec!["A", "C"], vec!["B", "D"]),
            (vec!["A", "D"], vec!["B", "C"]),
        ]);

        assert_eq!(actual.len(), expected.len());
        assert!(actual.iter().all(|x| expected.contains(x)));
    }
}
