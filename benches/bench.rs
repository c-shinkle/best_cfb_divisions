#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use best_cfb_divisions::pod::{algorithm::find_closest_pods, combo::get_all_pod_combinations};
    use test::Bencher;

    const POD_COUNT: usize = 3;
    const CONFERENCE: [&str; 12] = [
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
    ];

    #[bench]
    fn get_all_pod_quadruples_bench(b: &mut Bencher) {
        b.iter(|| {
            let _ = get_all_pod_combinations::<POD_COUNT>(&CONFERENCE);
        })
    }

    #[bench]
    fn find_closest_pods_bench(b: &mut Bencher) {
        b.iter(|| {
            let _ = find_closest_pods::<POD_COUNT>(&CONFERENCE);
        })
    }
}
