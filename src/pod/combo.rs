use crate::types::*;
use itertools::Itertools;

const POD_RATIO: usize = 4;
const PROTECTED_PAIRS: [TeamPair; 6] = [
    ("Indiana", "Purdue"),
    ("Michigan State", "Michigan"),
    ("Northwestern", "Illinois"),
    ("Ohio State", "Michigan"),
    ("Purdue", "Illinois"),
    ("Rutgers", "Maryland"),
    // ("UCLA", "USC"),
    // ("Iowa", "Minnesota"),
    // ("Iowa", "Nebraska"),
    // ("Iowa", "Wisconsin"),
    // ("Minnesota", "Wisconsin"),
];
pub type Pod = Vec<Team>;
pub type PodCombo<const N: usize> = [Pod; N];

pub fn get_all_pod_combinations<const POD_COUNT: usize>(
    conference: &Conference,
) -> Vec<PodCombo<POD_COUNT>> {
    assert!(
        conference.len() / POD_COUNT == POD_RATIO,
        "The ration between Conference length and Pod count must equal 4!"
    );

    helper::<POD_COUNT>(Vec::new(), conference.to_owned(), conference)
}

fn helper<const POD_COUNT: usize>(
    mut pods_so_far: Vec<Pod>,
    remaining_teams: Vec<Team>,
    conference: &Conference,
) -> Vec<PodCombo<POD_COUNT>> {
    if remaining_teams.len() <= POD_RATIO {
        pods_so_far.push(remaining_teams);
        let pod_combo: PodCombo<POD_COUNT> = pods_so_far
            .try_into()
            .expect("Pod count didn't match last pod");
        return vec![pod_combo];
    }
    let remaining_pods: Vec<Pod> = conference
        .iter()
        .copied()
        .filter(|team| remaining_teams.contains(team))
        .combinations(POD_RATIO)
        .collect();
    let mut all_pod_combos: Vec<PodCombo<POD_COUNT>> = Vec::new();

    for remaining_pod in remaining_pods {
        if !is_pod_valid(&remaining_pod) {
            continue;
        }
        let new_remaining_teams: Vec<Team> = remaining_teams
            .iter()
            .copied()
            .filter(|team| !remaining_pod.contains(team))
            .collect();

        let mut new_pods_so_far = pods_so_far.clone();
        new_pods_so_far.push(remaining_pod);

        let pod_combos = helper::<POD_COUNT>(new_pods_so_far, new_remaining_teams, conference);
        all_pod_combos.extend(pod_combos);
    }
    all_pod_combos
}

fn is_pod_valid(pod: &Pod) -> bool {
    for team in pod {
        let are_all_protected_pairs_in_pod = PROTECTED_PAIRS
            .into_iter()
            .filter(|(a, b)| a == team || b == team)
            .map(|(a, b)| if a == *team { b } else { a })
            .all(|opposing_team| pod.contains(&opposing_team));
        if !are_all_protected_pairs_in_pod {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pod_is_valid() {
        let pod = vec!["Indiana", "Purdue", "Illinois", "Northwestern"];

        assert!(is_pod_valid(&pod));
    }

    #[test]
    fn pod_is_not_valid() {
        let pod = vec!["Indiana", "Michigan State", "Northwestern", "Ohio State"];

        assert!(!is_pod_valid(&pod));
    }

    #[test]
    fn contains_some_pod() {
        let conference = [
            "Indiana",
            "Michigan State",
            "Northwestern",
            "Ohio State",
            "Penn State",
            "Purdue",
            "Rutgers",
            "UCLA",
            "Illinois",
            "Iowa",
            "Maryland",
            "Michigan",
            // "Minnesota",
            // "Nebraska",
            // "USC",
            // "Wisconsin",
        ];

        let pod_combos = get_all_pod_combinations::<3>(&conference);
        assert_eq!(pod_combos.len(), 7);
        let expected: Vec<PodCombo<3>> = vec![
            [
                vec!["Indiana", "Northwestern", "Purdue", "Illinois"],
                vec!["Ohio State", "Penn State", "Rutgers", "Maryland"],
                vec!["Michigan State", "UCLA", "Iowa", "Michigan"],
            ],
            [
                vec!["Indiana", "Northwestern", "Purdue", "Illinois"],
                vec!["Ohio State", "Penn State", "UCLA", "Iowa"],
                vec!["Michigan State", "Rutgers", "Maryland", "Michigan"],
            ],
            [
                vec!["Indiana", "Northwestern", "Purdue", "Illinois"],
                vec!["Ohio State", "Rutgers", "UCLA", "Maryland"],
                vec!["Michigan State", "Penn State", "Iowa", "Michigan"],
            ],
            [
                vec!["Indiana", "Northwestern", "Purdue", "Illinois"],
                vec!["Ohio State", "Rutgers", "Iowa", "Maryland"],
                vec!["Michigan State", "Penn State", "UCLA", "Michigan"],
            ],
            [
                vec!["Indiana", "Northwestern", "Purdue", "Illinois"],
                vec!["Penn State", "Rutgers", "UCLA", "Maryland"],
                vec!["Michigan State", "Ohio State", "Iowa", "Michigan"],
            ],
            [
                vec!["Indiana", "Northwestern", "Purdue", "Illinois"],
                vec!["Penn State", "Rutgers", "Iowa", "Maryland"],
                vec!["Michigan State", "Ohio State", "UCLA", "Michigan"],
            ],
            [
                vec!["Indiana", "Northwestern", "Purdue", "Illinois"],
                vec!["Rutgers", "UCLA", "Iowa", "Maryland"],
                vec!["Michigan State", "Ohio State", "Penn State", "Michigan"],
            ],
        ];
        for pod_combo in pod_combos {
            assert!(expected.contains(&pod_combo));
        }
    }
}
