pub mod division_pair_set {
    extern crate fxhash;
    use crate::types::*;
    use fxhash::{FxBuildHasher, FxHashSet};
    use itertools::Itertools;
    use std::collections::HashSet;
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
        let indexes_combinations: Vec<Vec<u32>> = (0..len as u32).combinations(len / 2).collect();
        let mut set: FxHashSet<Indexes> = HashSet::with_capacity_and_hasher(
            indexes_combinations.len() / 2,
            FxBuildHasher::default(),
        );

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
    extern crate fxhash;
    use crate::types::*;
    use fxhash::{FxBuildHasher, FxHashMap};
    use itertools::Itertools;

    pub fn get_all_division_pairs(conference: &Conference) -> Vec<DivisionPair> {
        let len = conference.len();
        let index_combinations: Vec<Vec<usize>> = (0..len).combinations(len / 2).collect();
        let mut map = FxHashMap::with_capacity_and_hasher(
            index_combinations.len() / 2,
            FxBuildHasher::default(),
        );
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
}
