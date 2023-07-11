use crate::types::*;
use itertools::Itertools;

pub fn get_all_pod_quadruples(conference: &Conference) -> Vec<PodQuadruple> {
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

    let mut all_pod_quadruples: Vec<PodQuadruple> = Vec::new();

    let first_pods: Vec<Pod> = conference.iter().copied().combinations(4).collect();
    let mut i = 0;
    for first_pod in first_pods.iter() {
        println!("iter {i}");
        i += 1;
        let available: Vec<Team> = conference
            .iter()
            .copied()
            .filter(|team| !first_pod.contains(team))
            .collect();
        assert!(available.len() == 12, "First filter != 12");
        let second_pods: Vec<Pod> = available.into_iter().combinations(4).collect();
        for second_pod in second_pods.iter() {
            let available: Vec<Team> = conference
                .iter()
                .copied()
                .filter(|team| !first_pod.contains(team) && !second_pod.contains(team))
                .collect();
            assert!(available.len() == 8, "Second filter != 8");
            let third_pods: Vec<Pod> = available.into_iter().combinations(4).collect();
            for third_pod in third_pods.iter() {
                let available: Vec<Team> = conference
                    .iter()
                    .copied()
                    .filter(|team| {
                        !first_pod.contains(team)
                            && !second_pod.contains(team)
                            && !third_pod.contains(team)
                    })
                    .collect();
                assert!(available.len() == 4, "Third filter != 4");
                let fourth_pod: Pod = available;
                all_pod_quadruples.push((
                    first_pod.clone(),
                    second_pod.clone(),
                    third_pod.clone(),
                    fourth_pod,
                ));
            }
        }
    }

    all_pod_quadruples
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
            "Minnesota",
            "Nebraska",
            "USC",
            "Wisconsin",
        ];

        let _ = get_all_pod_quadruples(&conference);
    }
}
