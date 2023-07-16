use std::sync::Arc;

use crate::types::{Conference, Team};
use itertools::Itertools;

use super::{Pod, PodTuple};

const POD_SIZE: usize = 4;

pub fn get_all_pod_combinations<const POD_COUNT: usize>(
    conference: &Conference,
) -> Vec<PodTuple<POD_COUNT>> {
    assert!(
        conference.len() / POD_COUNT == POD_SIZE,
        "Conference length divided by Pod coutn must equal 4!"
    );
    // let protected_pairs: Vec<TeamPair> = vec![
    //     ("Indiana University", "Purdue University"),
    //     ("Michigan State University", "University of Michigan"),
    //     ("Northwestern University", "University of Illinois"),
    //     ("Ohio State University", "University of Michigan"),
    //     ("Purdue University", "University of Illinois"),
    //     ("Rutgers University", "University of Maryland"),
    //     (
    //         "University of California, Los Angles",
    //         "University of Southern California",
    //     ),
    //     ("University of Iowa", "University of Minnesota"),
    //     ("University of Iowa", "University of Nebraska"),
    //     ("University of Iowa", "University of Wisconsin"),
    //     ("University of Minnesota", "University of Wisconsin"),
    // ];

    helper::<POD_COUNT>(Vec::new(), conference.to_owned(), conference)

    // let mut all_pod_quadruples: Vec<Vec<Pod>> = Vec::new();
    // let first_pods = get_list_of_remaining_quadruples(conference, |_| true);
    // for first in first_pods {
    //     let first = Arc::new(first);
    //     let second_pods =
    //         get_list_of_remaining_quadruples(conference, |team| !first.contains(team));
    //     for second in second_pods {
    //         let second = Arc::new(second);
    //         let third_pods = get_list_of_remaining_quadruples(conference, |team| {
    //             !first.contains(team) && !second.contains(team)
    //         });
    //         for third in third_pods {
    //             let third = Arc::new(third);
    //             let fourth: Pod = conference
    //                 .iter()
    //                 .copied()
    //                 .filter(|team| {
    //                     !first.contains(team) && !second.contains(team) && !third.contains(team)
    //                 })
    //                 .collect();
    //
    //             all_pod_quadruples.push([
    //                 first.clone(),
    //                 second.clone(),
    //                 third.clone(),
    //                 Arc::new(fourth),
    //             ]);
    //         }
    //     }
    // }
    //
    // all_pod_quadruples
}

fn helper<const POD_COUNT: usize>(
    mut pods_so_far: Vec<Arc<Pod>>,
    remaining_teams: Vec<Team>,
    conference: &Conference,
) -> Vec<PodTuple<POD_COUNT>> {
    // base case
    if remaining_teams.len() <= POD_SIZE {
        let last_pod: Pod = remaining_teams.into_iter().collect();
        pods_so_far.push(Arc::new(last_pod));
        return vec![PodTuple::<POD_COUNT>(
            pods_so_far
                .try_into()
                .expect("Pod count didn't match last pod"),
        )];
    }
    // recursive case
    let quadruples: Vec<Pod> =
        get_list_of_remaining_quadruples(conference, |team| remaining_teams.contains(team));
    let mut all_pod_quadruples: Vec<PodTuple<POD_COUNT>> = Vec::new();

    for quadruple in quadruples {
        let remaining_team_minus_quadruple: Vec<Team> = remaining_teams
            .iter()
            .copied()
            .filter(|team| !quadruple.contains(team))
            .collect();
        let mut pods: Vec<Arc<Pod>> = pods_so_far.clone();
        pods.push(Arc::new(quadruple));
        let mut foobar: Vec<PodTuple<POD_COUNT>> =
            helper::<POD_COUNT>(pods, remaining_team_minus_quadruple, conference);
        all_pod_quadruples.append(&mut foobar);
    }
    all_pod_quadruples
}

fn get_list_of_remaining_quadruples<F>(conference: &Conference, filter_predicate: F) -> Vec<Pod>
where
    F: Fn(&Team) -> bool,
{
    conference
        .iter()
        .copied()
        .filter(filter_predicate)
        .combinations(POD_SIZE)
        .collect()
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
