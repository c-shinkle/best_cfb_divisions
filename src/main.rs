use best_cfb_divisions::pod::combo::get_all_pod_combinations;
#[allow(unused_imports)]
use best_cfb_divisions::{
    division::algorithm::find_closest_divisions, pod::algorithm::find_closest_pods,
};

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
    // find_closest_pods::<4>(&conference);
    let all_pod_combos = get_all_pod_combinations::<3>(&conference);
    for combo in all_pod_combos {
        println!("{}", combo);
    }
}
