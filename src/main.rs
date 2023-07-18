use best_cfb_divisions::pod::algorithm::find_closest_pods;
#[allow(unused_imports)]
use best_cfb_divisions::pod::combo::get_all_pod_combinations;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::Write;

fn main() {
    let conference = [
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
        // "Minnesota",
        // "Nebraska",
        // "USC",
        // "Wisconsin",
    ];
    find_closest_pods::<3>(&conference);
    // let mut file = File::create("./all_three_pod_combos.txt").unwrap();
    // let all_pod_combos = get_all_pod_combinations::<3>(&conference);
    // for combo in all_pod_combos {
    //     let strings: Vec<String> = combo.into_iter().map(|pod| format!("{pod:?}")).collect();
    //     writeln!(file, "[{}]", strings.join(", ")).unwrap();
    // }
}
