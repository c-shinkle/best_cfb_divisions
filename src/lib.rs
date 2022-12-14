pub mod division_pairs;

use crate::division_pairs::get_all_division_pairs;
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;

type Conference = [&'static str];
type Division = Vec<&'static str>;
type TeamPair = (&'static str, &'static str);
type DivisionPair = (Division, Division);

#[derive(Clone, Eq, PartialEq)]
struct DivisionDistance {
    dist: u32,
    first: Division,
    second: Division,
}

impl PartialOrd<Self> for DivisionDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

impl Ord for DivisionDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
}

fn create_lookup_table() -> HashMap<TeamPair, u32> {
    collection! {
        ("Brigham Young University", "Iowa State University") => 1123,
        ("Brigham Young University", "Kansas State University") => 976,
        ("Brigham Young University", "Oklahoma State University") => 1120,
        ("Brigham Young University", "Texas Christian University") => 1178,
        ("Brigham Young University", "Texas Tech University") => 868,
        ("Brigham Young University", "University of Baylor") => 1259,
        ("Brigham Young University", "University of Central Florida") => 2339,
        ("Brigham Young University", "University of Cincinnati") => 1664,
        ("Brigham Young University", "University of Houston") => 1436,
        ("Brigham Young University", "University of Kansas") => 1047,
        ("Brigham Young University", "University of Oklahoma") => 1113,
        ("Brigham Young University", "University of Texas") => 1246,
        ("Brigham Young University", "University of West Virginia") => 1926,
        ("Iowa State University", "Kansas State University") => 356,
        ("Iowa State University", "Oklahoma State University") => 549,
        ("Iowa State University", "Texas Christian University") => 787,
        ("Iowa State University", "Texas Tech University") => 933,
        ("Iowa State University", "University of Baylor") => 869,
        ("Iowa State University", "University of Central Florida") => 1350,
        ("Iowa State University", "University of Cincinnati") => 566,
        ("Iowa State University", "University of Houston") => 978,
        ("Iowa State University", "University of Kansas") => 276,
        ("Iowa State University", "University of Oklahoma") => 604,
        ("Iowa State University", "University of Texas") => 969,
        ("Iowa State University", "University of West Virginia") => 828,
        ("Kansas State University", "Oklahoma State University") => 254,
        ("Kansas State University", "Texas Christian University") => 492,
        ("Kansas State University", "Texas Tech University") => 638,
        ("Kansas State University", "University of Baylor") => 574,
        ("Kansas State University", "University of Central Florida") => 1372,
        ("Kansas State University", "University of Cincinnati") => 708,
        ("Kansas State University", "University of Houston") => 735,
        ("Kansas State University", "University of Kansas") => 85,
        ("Kansas State University", "University of Oklahoma") => 309,
        ("Kansas State University", "University of Texas") => 674,
        ("Kansas State University", "University of West Virginia") => 983,
        ("Oklahoma State University", "Texas Christian University") => 267,
        ("Oklahoma State University", "Texas Tech University") => 413,
        ("Oklahoma State University", "University of Baylor") => 348,
        ("Oklahoma State University", "University of Central Florida") => 1311,
        ("Oklahoma State University", "University of Cincinnati") => 809,
        ("Oklahoma State University", "University of Houston") => 510,
        ("Oklahoma State University", "University of Kansas") => 281,
        ("Oklahoma State University", "University of Oklahoma") => 84,
        ("Oklahoma State University", "University of Texas") => 449,
        ("Oklahoma State University", "University of West Virginia") => 1084,
        ("Texas Christian University", "Texas Tech University") => 315,
        ("Texas Christian University", "University of Baylor") => 87,
        ("Texas Christian University", "University of Central Florida") => 1138,
        ("Texas Christian University", "University of Cincinnati") => 970,
        ("Texas Christian University", "University of Houston") => 267,
        ("Texas Christian University", "University of Kansas") => 516,
        ("Texas Christian University", "University of Oklahoma") => 187,
        ("Texas Christian University", "University of Texas") => 187,
        ("Texas Christian University", "University of West Virginia") => 1239,
        ("Texas Tech University", "University of Baylor") => 353,
        ("Texas Tech University", "University of Central Florida") => 1449,
        ("Texas Tech University", "University of Cincinnati") => 1193,
        ("Texas Tech University", "University of Houston") => 521,
        ("Texas Tech University", "University of Kansas") => 664,
        ("Texas Tech University", "University of Oklahoma") => 344,
        ("Texas Tech University", "University of Texas") => 373,
        ("Texas Tech University", "University of West Virginia") => 1468,
        ("University of Baylor", "University of Central Florida") => 1132,
        ("University of Baylor", "University of Cincinnati") => 1028,
        ("University of Baylor", "University of Houston") => 185,
        ("University of Baylor", "University of Kansas") => 598,
        ("University of Baylor", "University of Oklahoma") => 269,
        ("University of Baylor", "University of Texas") => 101,
        ("University of Baylor", "University of West Virginia") => 1297,
        ("University of Central Florida", "University of Cincinnati") => 915,
        ("University of Central Florida", "University of Houston") => 978,
        ("University of Central Florida", "University of Kansas") => 1294,
        ("University of Central Florida", "University of Oklahoma") => 1289,
        ("University of Central Florida", "University of Texas") => 1136,
        ("University of Central Florida", "University of West Virginia") => 900,
        ("University of Cincinnati", "University of Houston") => 1053,
        ("University of Cincinnati", "University of Kansas") => 629,
        ("University of Cincinnati", "University of Oklahoma") => 864,
        ("University of Cincinnati", "University of Texas") => 1130,
        ("University of Cincinnati", "University of West Virginia") => 306,
        ("University of Houston", "University of Kansas") => 760,
        ("University of Houston", "University of Oklahoma") => 431,
        ("University of Houston", "University of Texas") => 164,
        ("University of Houston", "University of West Virginia") => 1344,
        ("University of Kansas", "University of Oklahoma") => 333,
        ("University of Kansas", "University of Texas") => 698,
        ("University of Kansas", "University of West Virginia") => 903,
        ("University of Oklahoma", "University of Texas") => 369,
        ("University of Oklahoma", "University of West Virginia") => 1139,
        ("University of Texas", "University of West Virginia") => 1397,
    }
}

pub fn find_closest_divisions(conference: &Conference) {
    assert!(conference.len() >= 4);
    let all_division_pairs = get_all_division_pairs(conference);
    let lookup_table = create_lookup_table();
    let min_division_distance = all_division_pairs
        .into_par_iter()
        .map(|(first, second)| {
            let first_sum = sum_division_dist(&first, &lookup_table);
            let second_sum = sum_division_dist(&second, &lookup_table);
            let dist = (first_sum + second_sum) / conference.len() as u32;
            DivisionDistance {
                dist,
                first,
                second,
            }
        })
        .min()
        .unwrap();
    print_divisions(min_division_distance);
}

fn sum_division_dist(division: &Division, lookup_table: &HashMap<TeamPair, u32>) -> u32 {
    division
        .iter()
        .tuple_combinations::<(&&str, &&str)>()
        .map(|(a, b)| {
            *lookup_table
                .get(&(a, b))
                .or_else(|| lookup_table.get(&(b, a)))
                .unwrap()
        })
        .sum()
}

fn print_divisions(distance: DivisionDistance) {
    assert!(distance.first.len() > 0 && distance.second.len() > 0);
    println!("Distance: {}", distance.dist);
    print!("First Division: ");
    for i in 0..distance.first.len() - 1 {
        print!("{}, ", distance.first[i]);
    }
    println!("{}", distance.first[distance.first.len() - 1]);
    print!("Second Division: ");
    for i in 0..distance.second.len() - 1 {
        print!("{}, ", distance.second[i]);
    }
    println!("{}", distance.second[distance.second.len() - 1]);
}
