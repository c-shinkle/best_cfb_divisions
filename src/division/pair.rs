pub mod division_pair_set {
    use crate::types::*;
    use ahash::AHashSet;
    use itertools::Itertools;
    use std::hash::{Hash, Hasher};

    #[derive(Debug)]
    struct Indexes {
        indexes: Vec<u32>,
        compliment: Vec<u32>,
    }

    impl PartialEq for Indexes {
        fn eq(&self, other: &Indexes) -> bool {
            self.indexes == other.indexes || self.compliment == other.indexes
        }
    }

    impl Eq for Indexes {}

    impl Hash for Indexes {
        fn hash<H: Hasher>(&self, state: &mut H) {
            if self.indexes[0] == 0 {
                self.indexes.hash(state);
                self.compliment.hash(state);
            } else {
                self.compliment.hash(state);
                self.indexes.hash(state);
            }
        }
    }

    pub fn get_all_division_pairs(conference: &Conference) -> Vec<DivisionPair> {
        let len = conference.len();
        let number_of_combinations = count_combinations(len, len / 2);
        let mut indexes_combinations: Vec<Vec<u32>> = Vec::with_capacity(number_of_combinations);
        for combo in (0..len as u32).combinations(len / 2) {
            indexes_combinations.push(combo);
        }

        let mut set = AHashSet::with_capacity(number_of_combinations / 2);
        for indexes in indexes_combinations {
            let compliment = (0..len as u32).filter(|i| !indexes.contains(i)).collect();
            set.insert(Indexes {
                indexes,
                compliment,
            });
        }

        let index_into = |i: u32| conference[i as usize];
        set.into_iter()
            .map(|combo| {
                (
                    combo.indexes.into_iter().map(index_into).collect(),
                    combo.compliment.into_iter().map(index_into).collect(),
                )
            })
            .collect()
    }

    fn count_combinations(n: usize, r: usize) -> usize {
        (1..=r).fold(1, |acc, val| acc * (n - val + 1) / val)
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
            assert!(actual.iter().all(|x| expected.contains(x)));
        }
    }
}

pub mod division_pair_map {
    use crate::types::*;
    use ahash::AHashMap;
    use itertools::Itertools;

    #[allow(dead_code)]
    pub fn get_all_division_pairs(conference: &Conference) -> Vec<DivisionPair> {
        let len = conference.len();
        let number_of_combinations = count_combinations(len, len / 2);
        let mut index_combinations: Vec<Vec<usize>> = Vec::with_capacity(number_of_combinations);
        for combo in (0..len).combinations(len / 2) {
            index_combinations.push(combo);
        }
        let mut map = AHashMap::with_capacity(index_combinations.len() / 2);
        let index_into = |i: usize| conference[i];

        for combo in index_combinations {
            if !map.contains_key(&combo) {
                let comp: Vec<usize> = (0..len).filter(|i| !combo.contains(i)).collect();
                if !map.contains_key(&comp) {
                    let division_pair = (
                        combo.iter().copied().map(index_into).collect(),
                        comp.into_iter().map(index_into).collect(),
                    );
                    map.insert(combo, division_pair);
                }
            }
        }

        map.into_values().collect()
    }

    fn count_combinations(n: usize, r: usize) -> usize {
        (1..=r).fold(1, |acc, val| acc * (n - val + 1) / val)
    }
}
