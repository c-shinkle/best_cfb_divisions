mod script;
#[allow(unused_imports)]
use crate::script::write_all_stadium_distance_pairs_to_file;
use best_cfb_divisions::find_closest_divisions;

fn main() {
    let conference = [
        "Brigham Young University",
        "Iowa State University",
        "Kansas State University",
        "Oklahoma State University",
        "Texas Christian University",
        "Texas Tech University",
        "University of Baylor",
        "University of Central Florida",
        "University of Cincinnati",
        "University of Houston",
        "University of Kansas",
        "University of Oklahoma",
        "University of Texas",
        "University of West Virginia",
    ];
    // write_all_stadium_distance_pairs_to_file(&conference);
    find_closest_divisions(&conference);
}
