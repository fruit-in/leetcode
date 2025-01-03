use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut leftmost = HashMap::new();
        let mut rightmost = HashMap::new();
        let mut ret = 0;

        for &&x in nums.iter().collect::<HashSet<_>>().iter() {
            let lo = leftmost.remove(&(x - 1)).unwrap_or(x);
            let hi = rightmost.remove(&(x + 1)).unwrap_or(x);

            leftmost.insert(hi, lo);
            rightmost.insert(lo, hi);

            ret = ret.max(hi - lo + 1);
        }

        ret
    }
}
