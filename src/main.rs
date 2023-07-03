use best_cfb_divisions::division::algorithm::find_closest_divisions;

fn main() {
    let conference = [
        "Indiana University",
        "Michigan State University",
        "Northwestern University",
        "Ohio State University",
        "Pennsylvannia State University",
        "Purdue University",
        "Rutgers University",
        "University of California, Los Angles",
        "University of Illinois",
        "University of Iowa",
        "University of Maryland",
        "University of Michigan",
        "University of Minnesota",
        "University of Nebraska",
        "University of Southern California",
        "University of Wisconsin",
    ];
    find_closest_divisions(&conference);
}
