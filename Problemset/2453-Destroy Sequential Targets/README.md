# 2453. Destroy Sequential Targets
You are given a **0-indexed** array `nums` consisting of positive integers, representing targets on a number line. You are also given an integer `space`.

You have a machine which can destroy targets. **Seeding** the machine with some `nums[i]` allows it to destroy all targets with values that can be represented as `nums[i] + c * space`, where `c` is any non-negative integer. You want to destroy the **maximum** number of targets in `nums`.

Return *the **minimum value** of* `nums[i]` *you can seed the machine with to destroy the maximum number of targets*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,7,8,1,1,5], space = 2
<strong>Output:</strong> 1
<strong>Explanation:</strong> If we seed the machine with nums[3], then we destroy all targets equal to 1,3,5,7,9,...
In this case, we would destroy 5 total targets (all except for nums[2]).
It is impossible to destroy more than 5 targets, so we return nums[3].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,3,5,2,4,6], space = 2
<strong>Output:</strong> 1
<strong>Explanation:</strong> Seeding the machine with nums[0], or nums[3] destroys 3 targets.
It is not possible to destroy more than 3 targets.
Since nums[0] is the minimal integer that can destroy 3 targets, we return 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [6,2,5], space = 100
<strong>Output:</strong> 2
<strong>Explanation:</strong> Whatever initial seed we select, we can only destroy 1 target. The minimal seed is nums[1].
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>
* <code>1 <= space <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
        let mut map: HashMap<i32, (i32, i32)> = HashMap::new();
        let mut max_count = 0;
        let mut ret = 0;

        for i in 0..nums.len() {
            match map.get_mut(&(nums[i] % space)) {
                Some((min_num, count)) => {
                    *min_num = nums[i].min(*min_num);
                    *count += 1;
                    if max_count < *count || (max_count == *count && ret > *min_num) {
                        max_count = *count;
                        ret = *min_num;
                    }
                }
                None if max_count == 0 || (max_count == 1 && ret > nums[i]) => {
                    max_count = 1;
                    ret = nums[i];
                    map.insert(nums[i] % space, (nums[i], 1));
                }
                None => {
                    map.insert(nums[i] % space, (nums[i], 1));
                }
            }
        }

        ret
    }
}
```
