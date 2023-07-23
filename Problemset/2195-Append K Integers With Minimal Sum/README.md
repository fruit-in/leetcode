# 2195. Append K Integers With Minimal Sum
You are given an integer array `nums` and an integer `k`. Append `k` **unique positive** integers that do **not** appear in `nums` to `nums` such that the resulting total sum is **minimum**.

Return *the sum of the* `k` *integers appended to* `nums`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,4,25,10,25], k = 2
<strong>Output:</strong> 5
<strong>Explanation:</strong> The two unique positive integers that do not appear in nums which we append are 2 and 3.
The resulting sum of nums is 1 + 4 + 25 + 10 + 25 + 2 + 3 = 70, which is the minimum.
The sum of the two integers appended is 2 + 3 = 5, so we return 5.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [5,6], k = 6
<strong>Output:</strong> 25
<strong>Explanation:</strong> The six unique positive integers that do not appear in nums which we append are 1, 2, 3, 4, 7, and 8.
The resulting sum of nums is 5 + 6 + 1 + 2 + 3 + 4 + 7 + 8 = 36, which is the minimum.
The sum of the six integers appended is 1 + 2 + 3 + 4 + 7 + 8 = 25, so we return 25.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>
* <code>1 <= k <= 10<sup>8</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums;
        let mut k = k;
        let mut ret = 0;

        nums.push(0);
        nums.push(i32::MAX);
        nums.sort_unstable();

        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                continue;
            } else if nums[i] - nums[i - 1] - 1 < k {
                k -= nums[i] - nums[i - 1] - 1;
                ret +=
                    (nums[i - 1] as i64 + nums[i] as i64) * (nums[i] - nums[i - 1] - 1) as i64 / 2;
            } else {
                ret += (nums[i - 1] as i64 * 2 + 1 + k as i64) * k as i64 / 2;
                break;
            }
        }

        ret
    }
}
```
