#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use best_cfb_divisions::combo::get_all_division_pairs;
    use best_cfb_divisions::find_closest_divisions;
    use test::Bencher;

    const CONFERENCE: [&str; 12] = [
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

    #[bench]
    fn create_non_duplicate_combinations_test(b: &mut Bencher) {
        b.iter(|| {
            let _x = get_all_division_pairs(&CONFERENCE);
        })
    }

    #[bench]
    fn find_closest_divisions_test(b: &mut Bencher) {
        b.iter(|| {
            find_closest_divisions(&CONFERENCE);
        });
    }
}
