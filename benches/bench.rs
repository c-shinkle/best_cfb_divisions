#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use best_cfb_divisions::pod::combo::get_all_pod_combinations;
    use test::Bencher;

    const CONFERENCE: [&str; 16] = [
        "Indiana",
        "Michigan State",
        "Northwestern",
        "Ohio State",
        "Penn State",
        "Purdue",
        "Rutgers",
        "UCLA",
        "Illinois",
        "Iowa",
        "Maryland",
        "Michigan",
        "Minnesota",
        "Nebraska",
        "USC",
        "Wisconsin",
    ];

    #[bench]
    fn get_all_pod_quadruples_bench(b: &mut Bencher) {
        b.iter(|| {
            let _ = get_all_pod_combinations(&CONFERENCE);
        })
    }
}
