#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use best_cfb_divisions::division_pair::{division_pair_map, division_pair_set};
    use best_cfb_divisions::find_closest_divisions;
    use test::Bencher;

    const CONFERENCE: [&str; 14] = [
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

    #[bench]
    fn find_closest_divisions_bench(b: &mut Bencher) {
        b.iter(|| {
            find_closest_divisions(&CONFERENCE);
        });
    }

    #[bench]
    fn map_division_pairs_bench(b: &mut Bencher) {
        b.iter(|| {
            let _ = division_pair_map::get_all_division_pairs(&CONFERENCE);
        })
    }

    #[bench]
    fn set_division_pairs_bench(b: &mut Bencher) {
        b.iter(|| {
            let _ = division_pair_set::get_all_division_pairs(&CONFERENCE);
        })
    }
}
