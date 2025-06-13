use std::collections::HashSet;

impl Solution {
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut children = vec![vec![]; n];
        let mut sub_genetic = HashSet::new();
        let mut min_val = 1;
        let mut ans = vec![1; n];

        for i in 1..n {
            children[parents[i] as usize].push(i);
        }

        for i in 0..n {
            if nums[i] > 1 {
                continue;
            }

            let mut j = i as i32;

            while j != -1 {
                let mut stack = vec![j as usize];

                while let Some(k) = stack.pop() {
                    if !sub_genetic.contains(&nums[k]) {
                        sub_genetic.insert(nums[k]);
                        for &l in &children[k] {
                            stack.push(l);
                        }
                    }
                }

                while sub_genetic.contains(&min_val) {
                    min_val += 1;
                }
                ans[j as usize] = min_val;

                j = parents[j as usize];
            }

            break;
        }

        ans
    }
}
