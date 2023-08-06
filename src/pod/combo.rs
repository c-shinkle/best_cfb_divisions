use crate::types::*;
use itertools::Itertools;

// const PROTECTED_PAIRS: Vec<TeamPair> = vec![
//     ("Indiana", "Purdue"),
//     ("Iowa", "Minnesota"),
//     ("Iowa", "Nebraska"),
//     ("Iowa", "Wisconsin"),
//     ("Michigan State", "Michigan"),
//     ("Minnesota", "Wisconsin"),
//     ("Northwestern", "Illinois"),
//     ("Ohio State", "Michigan"),
//     ("Purdue", "Illinois"),
//     ("Rutgers", "Maryland"),
//     ("UCLA", "USC"),
// ];

#[derive(Debug)]
struct Schedule {
    team: Team,
    opponents: Vec<Team>,
}

pub fn get_all_combinations_two_play_opponents(conference: &Conference) {
    let len = conference.len();
    assert!(len % 4 == 0, "Conference must be divisible by 4!");

    let all_orderings = conference.iter().copied().permutations(len);
    let mut all_possible_schedules: Vec<Vec<Schedule>> = Vec::new();

    for ordering in all_orderings {
        let mut schedules: Vec<Schedule> = Vec::new();
        for i in 0..len {
            let current_team = ordering[i];
            let mut opponents = Vec::new();
            find_opponents_matched_with_current_team(&schedules, current_team, &mut opponents);

            let ordering_minus_i =
                ordering
                    .iter()
                    .enumerate()
                    .filter_map(|(j, &e)| if j != i { Some(e) } else { None });
            for team in ordering_minus_i {
                if opponents.len() == 3 {
                    schedules.push(Schedule {
                        team: current_team,
                        opponents,
                    });
                    break;
                }
                opponents.push(team);
            }
        }
        all_possible_schedules.push(schedules);
    }
}

fn find_opponents_matched_with_current_team(
    schedules: &Vec<Schedule>,
    current_team: Team,
    opponents: &mut Vec<Team>,
) {
    for schedule in schedules {
        if schedule.team == current_team {
            continue;
        };
        if let Some(opponent) = schedule
            .opponents
            .iter()
            .find(|opponent| **opponent == current_team)
        {
            opponents.push(opponent);
        }
        if opponents.len() == 3 {
            return;
        }
    }
}
