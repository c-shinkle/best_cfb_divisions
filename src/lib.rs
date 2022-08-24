use itertools::Itertools;
use std::collections::HashMap;
use std::iter::zip;

type Conference = [&'static str];
type Division = Vec<&'static str>;
type TeamPair = (&'static str, &'static str);
type DivisionPair = (Division, Division);

macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
}

pub fn find_closest_divisions() {
    let conference = [
        "Kansas State",
        "Central Florida",
        "Iowa State",
        "Cincinnati",
        "Kansas",
        "Houston",
    ];
    let all_divisions_pairs = generate_all_conferences(&conference);
    let lookup_table = create_lookup_table();
    let (min, (first, second)) =
        get_min_distance_and_division_pair(&all_divisions_pairs, &lookup_table);
    println!("The min average distance is: {} miles", min);
    print_divisions(first, second);
}

fn generate_all_conferences(conference: &Conference) -> Vec<DivisionPair> {
    let first_half = generate_first_half_divisions(conference);
    let second_half = first_half
        .iter()
        .map(|division| generate_complimentary_division(division, conference))
        .collect::<Vec<Division>>();
    zip(first_half, second_half)
        .map(|(first, second)| (first, second))
        .collect::<Vec<(Division, Division)>>()
}

fn generate_first_half_divisions(conference: &Conference) -> Vec<Division> {
    conference
        .iter()
        .combinations(conference.len() / 2)
        .map(|combos| combos.iter().map(|s| **s).collect::<Division>())
        .collect::<Vec<Division>>()
}

fn generate_complimentary_division(division: &Division, conference: &Conference) -> Division {
    conference
        .iter()
        .filter(|team| !division.contains(team))
        .copied()
        .collect::<Vec<&str>>()
}

fn create_lookup_table() -> HashMap<TeamPair, u32> {
    collection! {
        ("Central Florida", "Cincinnati") => 917,
        ("Central Florida", "Houston") => 981,
        ("Central Florida", "Iowa State") => 1377,
        ("Central Florida", "Kansas State") => 1378,
        ("Central Florida", "Kansas") => 1294,
        ("Cincinnati", "Houston") => 1054,
        ("Cincinnati", "Iowa State") => 598,
        ("Cincinnati", "Kansas State") => 712,
        ("Cincinnati", "Kansas") => 628,
        ("Houston", "Iowa State") => 973,
        ("Houston", "Kansas State") => 739,
        ("Houston", "Kansas") => 732,
        ("Iowa State", "Kansas State") => 333,
        ("Iowa State", "Kansas") => 267,
        ("Kansas State", "Kansas") => 87,
    }
}

fn get_min_distance_and_division_pair<'a>(
    all_divisions_pairs: &'a [DivisionPair],
    lookup_table: &HashMap<TeamPair, u32>,
) -> (f64, (&'a Division, &'a Division)) {
    println!("size: {}", all_divisions_pairs.len());
    all_divisions_pairs.iter().fold(
        (
            f64::INFINITY,
            (&all_divisions_pairs[0].0, &all_divisions_pairs[0].1),
        ),
        |min_so_far, division_pair| {
            let (first, second) = division_pair;
            let length = (first.len() + second.len()) as u32;
            let first_sum = sum_division_dist(first, lookup_table);
            let second_sum = sum_division_dist(second, lookup_table);
            let current_min = (first_sum + second_sum) / f64::from(length);
            if current_min < min_so_far.0 {
                println!("New min found!");
                print_divisions(first, second);
                (current_min, (first, second))
            } else {
                min_so_far
            }
        },
    )
}

fn sum_division_dist(division: &Division, lookup_table: &HashMap<TeamPair, u32>) -> f64 {
    let sum = division
        .iter()
        .tuple_combinations::<(&&str, &&str)>()
        .map(|(a, b)| {
            *lookup_table
                .get(&(a, b))
                .or_else(|| lookup_table.get(&(b, a)))
                .unwrap()
        })
        .sum::<u32>();
    f64::from(sum)
}

fn print_divisions(first: &Division, second: &Division) {
    print!("First Division: ");
    print!("{}, ", first[0]);
    print!("{}, ", first[1]);
    print!("{}", first[2]);
    println!();
    print!("Second Division: ");
    print!("{}, ", second[0]);
    print!("{}, ", second[1]);
    print!("{}", second[2]);
    println!();
}

pub fn generate_all_pair_combinations(conference: &Conference) -> Vec<(&str, &str)> {
    conference
        .iter()
        .tuple_combinations::<(&&str, &&str)>()
        .map(|(a, b)| (*a, *b))
        .collect()
}
