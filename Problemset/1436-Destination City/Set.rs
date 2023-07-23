use std::collections::HashSet;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let city_a = paths.iter().map(|path| &path[0]).collect::<HashSet<_>>();

        paths
            .iter()
            .find(|path| !city_a.contains(&path[1]))
            .unwrap()[1]
            .clone()
    }
}
