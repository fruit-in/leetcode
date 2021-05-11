use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut nums = vec![(n as i64, 0)].into_iter().collect::<VecDeque<_>>();
        let mut seen = vec![n as i64].into_iter().collect::<HashSet<_>>();

        while let Some((x, y)) = nums.pop_front() {
            if x == 1 {
                return y;
            }
            if x % 2 == 0 && !seen.contains(&(x / 2)) {
                nums.push_back((x / 2, y + 1));
                seen.insert(x / 2);
            } else if x % 2 == 1 {
                if !seen.contains(&(x + 1)) {
                    nums.push_back((x + 1, y + 1));
                    seen.insert(x + 1);
                }
                if !seen.contains(&(x - 1)) {
                    nums.push_back((x - 1, y + 1));
                    seen.insert(x - 1);
                }
            }
        }

        0
    }
}
