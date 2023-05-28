use crate::types::DivisionPair;
use crate::Conference;
use itertools::Itertools;
use std::collections::HashSet;

pub fn get_all_division_pairs(conference: &Conference) -> Vec<DivisionPair> {
    let len = conference.len();
    let index_combinations: Vec<Vec<usize>> = (0..len).combinations(len / 2).collect();
    let division_pairs_len = index_combinations.len() / 2;

    let mut set: HashSet<Vec<usize>> = HashSet::with_capacity(division_pairs_len);
    let index_into = |i: usize| conference[i];

    for index_combo in index_combinations {
        if !set.contains(&index_combo) {
            let comp = (0..len).filter(|i| !index_combo.contains(i)).collect_vec();
            if !set.contains(&comp) {
                set.insert(index_combo);
            }
        }
    }

    set.into_iter()
        .map(|index_combo| {
            (
                index_combo.iter().copied().map(index_into).collect(),
                (0..len)
                    .filter(|i| !index_combo.contains(i))
                    .map(index_into)
                    .collect(),
            )
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

        let expected: HashSet<DivisionPair> = HashSet::from([
            (vec!["A", "B"], vec!["C", "D"]),
            (vec!["A", "C"], vec!["B", "D"]),
            (vec!["A", "D"], vec!["B", "C"]),
        ]);

        assert_eq!(actual.len(), expected.len());
        println!("{:?}", actual);
        assert!(actual.iter().all(|x| expected.contains(x)));
    }
}
