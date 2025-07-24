# 2009. Minimum Number of Operations to Make Array Continuous
You are given an integer array `nums`. In one operation, you can replace **any** element in `nums` with **any** integer.

`nums` is considered **continuous** if both of the following conditions are fulfilled:
* All elements in `nums` are **unique**.
* The difference between the **maximum** element and the **minimum** element in `nums` equals `nums.length - 1`.

For example, `nums = [4, 2, 5, 3]` is **continuous**, but `nums = [1, 2, 3, 5, 6]` is **not continuous**.

Return *the **minimum** number of operations to make* `nums` ***continuous***.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [4,2,5,3]
<strong>Output:</strong> 0
<strong>Explanation:</strong> nums is already continuous.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,5,6]
<strong>Output:</strong> 1
<strong>Explanation:</strong> One possible solution is to change the last element to 4.
The resulting array is [1,2,3,5,4], which is continuous.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,10,100,1000]
<strong>Output:</strong> 3
<strong>Explanation:</strong> One possible solution is to:
- Change the second element to 2.
- Change the third element to 3.
- Change the fourth element to 4.
The resulting array is [1,2,3,4], which is continuous.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut i = 0;
        let mut count = 0;
        let mut ret = n;

        nums.sort_unstable();

        for j in 0..nums.len() {
            if j == 0 || nums[j] != nums[j - 1] {
                count += 1;
            }
            while nums[j] - nums[i] > n - 1 {
                if i == 0 || nums[i] != nums[i - 1] {
                    count -= 1;
                }
                i += 1;
            }

            ret = ret.min(n - count);
        }

        ret
    }
}
```
