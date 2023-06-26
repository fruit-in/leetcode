use std::collections::HashSet;

impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let banned = banned.into_iter().collect::<HashSet<_>>();
        let mut sum = 0;
        let mut ret = 0;

        for x in 1..=n {
            if !banned.contains(&x) && sum + x <= max_sum {
                sum += x;
                ret += 1;
            } else if sum + x > max_sum {
                break;
            }
        }

        ret
    }
}
