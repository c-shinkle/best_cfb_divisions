#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use best_cfb_divisions::pod::{
        algorithm::find_best_two_play_opponents, combo::get_all_combinations_two_play_opponents,
    };
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
            let _ = get_all_combinations_two_play_opponents::<POD_COUNT>(&CONFERENCE);
        })
    }

    #[bench]
    fn find_closest_pods_bench(b: &mut Bencher) {
        b.iter(|| {
            let _ = find_best_two_play_opponents::<POD_COUNT>(&CONFERENCE);
        })
    }
}
