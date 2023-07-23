impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut version1 = version1
            .split('.')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>();
        let mut version2 = version2
            .split('.')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>();

        while let Some(&0) = version1.last() {
            version1.pop();
        }
        while let Some(&0) = version2.last() {
            version2.pop();
        }

        match version1.cmp(&version2) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
        }
    }
}
