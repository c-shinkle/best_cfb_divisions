#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use best_cfb_divisions::find_closest_divisions;
    use test::Bencher;

    #[bench]
    fn test(b: &mut Bencher) {
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
        b.iter(|| {
            find_closest_divisions(&conference);
        });
    }
}
