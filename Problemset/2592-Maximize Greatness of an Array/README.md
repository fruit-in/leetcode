# 2592. Maximize Greatness of an Array
You are given a 0-indexed integer array `nums`. You are allowed to permute `nums` into a new array `perm` of your choosing.

We define the **greatness** of `nums` be the number of indices `0 <= i < nums.length` for which `perm[i] > nums[i]`.

Return *the **maximum** possible greatness you can achieve after permuting* `nums`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,3,5,2,1,3,1]
<strong>Output:</strong> 4
<strong>Explanation:</strong> One of the optimal rearrangements is perm = [2,5,1,3,3,1,1].
At indices = 0, 1, 3, and 4, perm[i] > nums[i]. Hence, we return 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,4]
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can prove the optimal perm is [2,3,4,1].
At indices = 0, 1, and 2, perm[i] > nums[i]. Hence, we return 3.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximize_greatness(mut nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut ret = 0;

        nums.sort_unstable();

        for j in 0..nums.len() {
            while i < nums.len() && nums[i] <= nums[j] {
                i += 1;
            }

            if i >= nums.len() {
                break;
            } else {
                i += 1;
                ret += 1;
            }
        }

        ret
    }
}
```
