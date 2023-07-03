// use itertools::Itertools;
// use reqwest::blocking::Client;
// use serde_json::Value;
// use std::fs::File;
// use std::io::Write;

// #[allow(dead_code)]
// pub fn write_all_stadium_distance_pairs_to_file(stadium_names: &[&str]) {
//     let client = Client::new();
//     let mut file = File::create("output.txt").expect("creating file failed!");

//     let combinations = stadium_names
//         .iter()
//         .tuple_combinations::<(&&str, &&str)>()
//         .collect::<Vec<(&&str, &&str)>>();
//     for (origin, destination) in combinations {
//         let params = [
//             ("outputFormat", "json"),
//             ("origin", origin),
//             ("destination", destination),
//             ("key", "<key>"),
//         ];
//         let request = client
//             .get("https://maps.googleapis.com/maps/api/directions/json")
//             .query(&params)
//             .send()
//             .expect("The web client failed!");

//         if !request.status().is_success() {
//             panic!("Received status code {}", request.status().as_str());
//         }

//         let json = request.text().expect("The json failed!");
//         // println!("Response Body: {}", json);

//         let value = serde_json::from_str::<Value>(&json).expect("serde_json failed!");
//         let routes = value
//             .get("routes")
//             .and_then(|routes| routes.as_array())
//             .expect("No 'routes' found!");
//         let route = routes
//             .get(0)
//             .and_then(|route| route.as_object())
//             .expect("No 'route' found!");
//         let legs = route
//             .get("legs")
//             .and_then(|legs| legs.as_array())
//             .expect("No 'legs' found!");
//         let leg = legs
//             .get(0)
//             .and_then(|leg| leg.as_object())
//             .expect("No 'leg' found!");
//         let distance = leg.get("distance").expect("No 'distance' found!");
//         let text = distance.get("text").expect("No 'text' found!");
//         let miles_string = text.to_string();
//         let raw_miles = miles_string
//             .trim_matches('"')
//             .split(' ')
//             .next()
//             .expect(&format!("Couldn't parse: {}", miles_string))
//             .replace(',', "");
//         println!("The distance from {origin} to {destination} is {raw_miles} miles!");

//         let miles = raw_miles
//             .parse::<f64>()
//             .unwrap_or_else(|error| panic!("{error}"));
//         let temp = format!("(\"{}\", \"{}\") => {:.0},\n", origin, destination, miles);
//         file.write_all(temp.as_bytes())
//             .expect("file writing failed!");
//     }
// }
