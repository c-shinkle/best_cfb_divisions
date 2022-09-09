pub mod division_pairs;
use crate::division_pairs::get_all_division_pairs;
use itertools::Itertools;
use std::collections::HashMap;

type Conference = [&'static str];
type Division = Vec<&'static str>;
type TeamPair = (&'static str, &'static str);

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
struct DivisionDistance {
    dist: u32,
    first: Division,
    second: Division,
}

macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
}

fn create_lookup_table() -> HashMap<TeamPair, u32> {
    collection! {
        ("Brigham Young University", "Iowa State University") => 1115,
        ("Brigham Young University", "Kansas State University") => 979,
        ("Brigham Young University", "Oklahoma State University") => 1122,
        ("Brigham Young University", "Texas Christian University") => 1180,
        ("Brigham Young University", "Texas Tech University") => 873,
        ("Brigham Young University", "University of Baylor") => 1261,
        ("Brigham Young University", "University of Central Florida") => 2339,
        ("Brigham Young University", "University of Cincinnati") => 1663,
        ("Brigham Young University", "University of Houston") => 1439,
        ("Brigham Young University", "University of Kansas") => 1049,
        ("Brigham Young University", "University of West Virginia") => 1926,
        ("Iowa State University", "Kansas State University") => 351,
        ("Iowa State University", "Oklahoma State University") => 542,
        ("Iowa State University", "Texas Christian University") => 780,
        ("Iowa State University", "Texas Tech University") => 926,
        ("Iowa State University", "University of Baylor") => 862,
        ("Iowa State University", "University of Central Florida") => 1389,
        ("Iowa State University", "University of Cincinnati") => 596,
        ("Iowa State University", "University of Houston") => 972,
        ("Iowa State University", "University of Kansas") => 267,
        ("Iowa State University", "University of West Virginia") => 859,
        ("Kansas State University", "Oklahoma State University") => 256,
        ("Kansas State University", "Texas Christian University") => 493,
        ("Kansas State University", "Texas Tech University") => 640,
        ("Kansas State University", "University of Baylor") => 575,
        ("Kansas State University", "University of Central Florida") => 1376,
        ("Kansas State University", "University of Cincinnati") => 710,
        ("Kansas State University", "University of Houston") => 738,
        ("Kansas State University", "University of Kansas") => 87,
        ("Kansas State University", "University of West Virginia") => 986,
        ("Oklahoma State University", "Texas Christian University") => 268,
        ("Oklahoma State University", "Texas Tech University") => 414,
        ("Oklahoma State University", "University of Baylor") => 350,
        ("Oklahoma State University", "University of Central Florida") => 1312,
        ("Oklahoma State University", "University of Cincinnati") => 809,
        ("Oklahoma State University", "University of Houston") => 512,
        ("Oklahoma State University", "University of Kansas") => 279,
        ("Oklahoma State University", "University of West Virginia") => 1085,
        ("Texas Christian University", "Texas Tech University") => 321,
        ("Texas Christian University", "University of Baylor") => 87,
        ("Texas Christian University", "University of Central Florida") => 1139,
        ("Texas Christian University", "University of Cincinnati") => 970,
        ("Texas Christian University", "University of Houston") => 268,
        ("Texas Christian University", "University of Kansas") => 516,
        ("Texas Christian University", "University of West Virginia") => 1242,
        ("Texas Tech University", "University of Baylor") => 355,
        ("Texas Tech University", "University of Central Florida") => 1453,
        ("Texas Tech University", "University of Cincinnati") => 1191,
        ("Texas Tech University", "University of Houston") => 524,
        ("Texas Tech University", "University of Kansas") => 662,
        ("Texas Tech University", "University of West Virginia") => 1467,
        ("University of Baylor", "University of Central Florida") => 1158,
        ("University of Baylor", "University of Cincinnati") => 1028,
        ("University of Baylor", "University of Houston") => 186,
        ("University of Baylor", "University of Kansas") => 598,
        ("University of Baylor", "University of West Virginia") => 1300,
        ("University of Central Florida", "University of Cincinnati") => 917,
        ("University of Central Florida", "University of Houston") => 981,
        ("University of Central Florida", "University of Kansas") => 1294,
        ("University of Central Florida", "University of West Virginia") => 901,
        ("University of Cincinnati", "University of Houston") => 1054,
        ("University of Cincinnati", "University of Kansas") => 627,
        ("University of Cincinnati", "University of West Virginia") => 307,
        ("University of Houston", "University of Kansas") => 760,
        ("University of Houston", "University of West Virginia") => 1348,
        ("University of Kansas", "University of West Virginia") => 902,
    }
}

pub fn find_closest_divisions(conference: &Conference) {
    let all_division_pairs = get_all_division_pairs(conference);
    let lookup_table = create_lookup_table();
    let length = conference.len() as u32;
    all_division_pairs
        .into_iter()
        .map(|(first, second)| {
            let first_sum = sum_division_dist(&first, &lookup_table);
            let second_sum = sum_division_dist(&second, &lookup_table);
            let dist = (first_sum + second_sum) / length;
            DivisionDistance {
                dist,
                first,
                second,
            }
        })
        .sorted_unstable()
        .for_each(print_divisions);
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
    let DivisionDistance {
        dist,
        first,
        second,
    } = distance;
    println!("Distance: {}", dist);
    print!("First Division: ");
    first.into_iter().for_each(|d| print!("{}, ", d));
    println!();
    print!("Second Division: ");
    second.into_iter().for_each(|d| print!("{}, ", d));
    print!("\n\n");
}

/*
pub fn write_all_stadium_distance_pairs_to_file(stadium_names: &[&'static str]) {
    let client = Client::new();
    let mut file = File::create("output.txt").expect("creating file failed!");

    let combinations = stadium_names
        .iter()
        .tuple_combinations::<(&&str, &&str)>()
        .map(|(a, b)| (*a, *b))
        .collect::<Vec<(&str, &str)>>();
    for (origin, destination) in combinations {
        let params = [
            ("outputFormat", "json"),
            ("origin", origin),
            ("destination", destination),
            ("key", "<key>"),
        ];
        let request = client
            .get("https://maps.googleapis.com/maps/api/directions/json")
            .query(&params)
            .send()
            .expect("The web client failed!");

        if !request.status().is_success() {
            println!("Received status code {}", request.status().as_str());
            return;
        }

        let json = request.text().expect("The json failed!");
        // println!("Response Body: {}", json);

        let value = serde_json::from_str::<Value>(&json).expect("serde_json failed!");
        let routes = value["routes"].as_array().expect("No 'routes' found!");
        let route = routes[0].as_object().expect("No 'route' found!");
        let legs = route
            .get("legs")
            .unwrap()
            .as_array()
            .expect("No 'legs' found!");
        let leg = legs[0].as_object().expect("No 'leg' found!");
        let distance = leg.get("distance").expect("No 'distance' found!");
        let text = distance.get("text").expect("No 'text' found!");
        let miles_string = text.to_string();
        let miles = miles_string
            .trim_matches('"')
            .split(' ')
            .next()
            .unwrap_or_else(|| panic!("Couldn't parse: {}", miles_string));
        println!(
            "The distance from {} to {} is {} miles!",
            origin, destination, miles
        );

        let temp = format!("(\"{}\", \"{}\") => {},\n", origin, destination, miles);
        file.write_all(temp.as_bytes())
            .expect("file writing failed!");
    }
}
 */
