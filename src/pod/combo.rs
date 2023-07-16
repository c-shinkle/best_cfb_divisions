use std::sync::Arc;

use crate::types::*;
use itertools::Itertools;

const POD_RATIO: usize = 4;
pub type Pod = Vec<Team>;
pub type PodCombo<const N: usize> = [Arc<Pod>; N];

pub fn get_all_pod_combinations<const POD_COUNT: usize>(
    conference: &Conference,
) -> Vec<PodCombo<POD_COUNT>> {
    assert!(
        conference.len() / POD_COUNT == POD_RATIO,
        "The ration between Conference length and Pod count must equal 4!"
    );
    // let protected_pairs: Vec<TeamPair> = vec![
    //     ("Indiana", "Purdue"),
    //     ("Michigan State", "Michigan"),
    //     ("Northwestern ", "Illinois"),
    //     ("Ohio State ", "Michigan"),
    //     ("Purdue", "Illinois"),
    //     ("Rutgers", "Maryland"),
    //     ("UCLA", "USC"),
    //     ("Iowa", "Minnesota"),
    //     ("Iowa", "Nebraska"),
    //     ("Iowa", "Wisconsin"),
    //     ("Minnesota", "Wisconsin"),
    // ];

    helper::<POD_COUNT>(Vec::new(), conference.to_owned(), conference)
}

fn helper<const POD_COUNT: usize>(
    mut pods_so_far: Vec<Arc<Pod>>,
    remaining_teams: Vec<Team>,
    conference: &Conference,
) -> Vec<PodCombo<POD_COUNT>> {
    if remaining_teams.len() <= POD_RATIO {
        pods_so_far.push(Arc::new(remaining_teams));
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
        let new_remaining_teams: Vec<Team> = remaining_teams
            .iter()
            .copied()
            .filter(|team| !remaining_pod.contains(team))
            .collect();

        let mut new_pods_so_far = pods_so_far.clone();
        new_pods_so_far.push(Arc::new(remaining_pod));

        let pod_combos = helper::<POD_COUNT>(new_pods_so_far, new_remaining_teams, conference);
        all_pod_combos.extend(pod_combos);
    }
    all_pod_combos
}

#[cfg(test)]
mod test {
    use super::*;

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

        let _ = get_all_pod_combinations::<3>(&conference);
    }
}
