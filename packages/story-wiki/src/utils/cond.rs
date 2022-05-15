use std::collections::HashSet;

pub fn should_show(
    observed_releases_references: &HashSet<&str>,
    cond: &str,
    continuity_prefix: &str,
) -> bool {
    let (prefix_mode, tag) = cond
        .split_once('-')
        .expect(&format!("Invalid condition: {cond}"));
    let in_continuity = tag.starts_with(continuity_prefix);
    match prefix_mode {
        "x" => {
            if in_continuity {
                !observed_releases_references.contains(&tag)
            } else {
                true
            }
        }
        "o" => {
            if in_continuity {
                observed_releases_references.contains(&tag)
            } else {
                true
            }
        }
        "xx" => !observed_releases_references.contains(&tag),
        "oo" => observed_releases_references.contains(&tag),
        _ => panic!("Invalid condition: {cond}"),
    }
}
