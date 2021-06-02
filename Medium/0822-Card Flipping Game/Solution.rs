use std::collections::HashSet;

impl Solution {
    pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
        let mut good = HashSet::new();
        let mut ban = HashSet::new();

        for i in 0..backs.len() {
            if fronts[i] == backs[i] {
                good.remove(&backs[i]);
                ban.insert(backs[i]);
            } else {
                if !ban.contains(&fronts[i]) {
                    good.insert(fronts[i]);
                }
                if !ban.contains(&backs[i]) {
                    good.insert(backs[i]);
                }
            }
        }

        good.into_iter().min().unwrap_or(0)
    }
}
