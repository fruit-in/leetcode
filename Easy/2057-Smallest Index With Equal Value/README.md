# 2057. Smallest Index With Equal Value
Given a **0-indexed** integer array `nums`, return *the **smallest** index* `i` *of* `nums` *such that* `i mod 10 == nums[i]`, *or* `-1` *if such index does not exist*.

`x mod y` denotes the **remainder** when `x` is divided by `y`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [0,1,2]
<strong>Output:</strong> 0
<strong>Explanation:</strong>
i=0: 0 mod 10 = 0 == nums[0].
i=1: 1 mod 10 = 1 == nums[1].
i=2: 2 mod 10 = 2 == nums[2].
All indices have i mod 10 == nums[i], so we return the smallest index 0.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [4,3,2,1]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
i=0: 0 mod 10 = 0 != nums[0].
i=1: 1 mod 10 = 1 != nums[1].
i=2: 2 mod 10 = 2 == nums[2].
i=3: 3 mod 10 = 3 != nums[3].
2 is the only index which has i mod 10 == nums[i].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,5,6,7,8,9,0]
<strong>Output:</strong> -1
<strong>Explanation:</strong> No index satisfies i mod 10 == nums[i].
</pre>

#### Constraints:
* `1 <= nums.length <= 100`
* `0 <= nums[i] <= 9`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        (0..nums.len())
            .find(|&i| i as i32 % 10 == nums[i])
            .map_or(-1, |x| x as i32)
    }
}
```
