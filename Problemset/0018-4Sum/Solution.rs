use std::collections::HashSet;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let target = target as i64;
        let mut nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let mut ret = HashSet::new();

        nums.sort_unstable();

        for a in 0..nums.len() {
            for b in a + 1..nums.len() {
                let mut c = b + 1;
                let mut d = nums.len() - 1;

                while c < d {
                    let sum = nums[a] + nums[b] + nums[c] + nums[d];

                    if sum == target {
                        let mut quadruplets = vec![
                            nums[a] as i32,
                            nums[b] as i32,
                            nums[c] as i32,
                            nums[d] as i32,
                        ];
                        quadruplets.sort_unstable();
                        ret.insert(quadruplets);
                        c += 1;
                        d -= 1;
                    } else if sum < target {
                        c += 1;
                    } else {
                        d -= 1;
                    }
                }
            }
        }

        ret.into_iter().collect()
    }
}
