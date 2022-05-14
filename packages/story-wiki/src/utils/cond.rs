use std::collections::HashSet;

pub fn should_show(observed_releases_references: &HashSet<&str>, cond: &str) -> bool {
    match &cond[..2] {
        "x-" => !observed_releases_references.contains(&cond[2..]),
        "o-" => observed_releases_references.contains(&cond[2..]),
        _ => panic!("Invalid condition: {cond}"),
    }
}
