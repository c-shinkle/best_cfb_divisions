mod script;
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
    find_closest_divisions(&conference);
}
