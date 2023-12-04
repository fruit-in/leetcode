# 18. 4Sum
Given an array `nums` of `n` integers, return *an array of all the **unique** quadruplets* `[nums[a], nums[b], nums[c], nums[d]]` such that:

* `0 <= a, b, c, d < n`
* `a`, `b`, `c`, and `d` are **distinct**.
* `nums[a] + nums[b] + nums[c] + nums[d] == target`

You may return the answer in **any order**.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,0,-1,0,-2,2], target = 0
<strong>Output:</strong> [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,2,2,2,2], target = 8
<strong>Output:</strong> [[2,2,2,2]]
</pre>

#### Constraints:
* `1 <= nums.length <= 200`
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>
* <code>-10<sup>9</sup> <= target <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
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
```
