use lib::find_closest_divisions;

mod lib;

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
        "University of West Virginia",
    ];
    find_closest_divisions(&conference);
}
