use reqwest::blocking::Client;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

use lib::find_closest_divisions;
use lib::generate_all_pair_combinations;

mod lib;

fn main() {
    find_closest_divisions();
    let stadium_names = [
        "Brigham Young University Stadium",
        "Iowa State University Stadium",
        "Kansas State University Stadium",
        "Oklahoma State University Stadium",
        "Texas Christian University Stadium",
        "Texas Tech University Stadium",
        "University of Baylor Stadium",
        "University of Central Florida Stadium",
        "University of Cincinnati Stadium",
        "University of Houston Stadium",
        "University of Kansas Stadium",
        "University of West Virginia Stadium",
    ];

    let client = Client::new();
    let mut file = File::create("output.txt").expect("creating file failed!");

    for (origin, destination) in generate_all_pair_combinations(&stadium_names) {
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

        let json = request.text().expect("The json failed!");

        let value = serde_json::from_str::<Value>(&json).expect("serde_json failed!");
        let routes = value["routes"].as_array().unwrap();
        let route = routes[0].as_object().unwrap();
        let legs = route.get("legs").unwrap().as_array().unwrap();
        let leg = legs[0].as_object().unwrap();
        let distance = leg.get("distance").unwrap();
        let text = distance.get("text").unwrap();
        let miles_string = text.to_string();
        let miles_token = miles_string.trim_matches('"').split(' ').next().unwrap();
        println!(
            "The distance from {} to {} is {} miles!",
            origin, destination, miles_token
        );
        //("Central Florida", "Cincinnati") => 917,
        let temp = format!(
            "(\"{}\", \"{}\") => {},\n",
            origin, destination, miles_token
        );
        file.write_all(temp.as_bytes())
            .expect("file writing failed!");
    }
}
