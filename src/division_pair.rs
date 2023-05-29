use crate::types::DivisionPair;
use crate::Conference;
use itertools::Itertools;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct CombinationIndexes {
    indexes: Vec<usize>,
}

impl PartialEq for CombinationIndexes {
    fn eq(&self, other: &CombinationIndexes) -> bool {
        self.indexes == other.indexes
            || get_compliment(self.indexes.len() * 2, &self.indexes) == other.indexes
    }
}

impl Eq for CombinationIndexes {}

impl Hash for CombinationIndexes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let comp = get_compliment(self.indexes.len() * 2, &self.indexes);
        if self.indexes[0] == 0 {
            self.indexes.hash(state);
            comp.hash(state);
        } else {
            comp.hash(state);
            self.indexes.hash(state);
        }
    }
}

fn get_compliment(len: usize, index_combo: &[usize]) -> Vec<usize> {
    (0..len).filter(|i| !index_combo.contains(i)).collect()
}

pub fn get_all_division_pairs(conference: &Conference) -> Vec<DivisionPair> {
    let len = conference.len();
    let indexes_combinations: Vec<Vec<usize>> = (0..len).combinations(len / 2).collect();

    let mut set: HashSet<CombinationIndexes> =
        HashSet::with_capacity(indexes_combinations.len() / 2);

    for indexes in indexes_combinations {
        set.insert(CombinationIndexes { indexes });
    }

    let index_into = |i: usize| conference[i];
    set.into_iter()
        .map(|combo| {
            let indexes = combo.indexes;
            (
                (0..len)
                    .filter(|i| !indexes.contains(i))
                    .map(index_into)
                    .collect(),
                indexes.into_iter().map(index_into).collect(),
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
            (vec!["C", "D"], vec!["A", "B"]),
            (vec!["B", "D"], vec!["A", "C"]),
            (vec!["B", "C"], vec!["A", "D"]),
        ]);

        assert_eq!(actual.len(), expected.len());
        assert!(actual.iter().all(|x| expected.contains(x)));
    }
}
