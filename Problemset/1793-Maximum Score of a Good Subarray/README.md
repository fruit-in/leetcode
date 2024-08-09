# 1793. Maximum Score of a Good Subarray
You are given an array of integers `nums` (**0-indexed**) and an integer `k`.

The **score** of a subarray `(i, j)` is defined as `min(nums[i], nums[i+1], ..., nums[j]) * (j - i + 1)`. A **good** subarray is a subarray where `i <= k <= j`.

Return *the maximum possible **score** of a **good** subarray*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,4,3,7,4,5], k = 3
<strong>Output:</strong> 15
<strong>Explanation:</strong> The optimal subarray is (1, 5) with a score of min(4,3,7,4,5) * (5-1+1) = 3 * 5 = 15.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [5,5,4,5,4,1,1,1], k = 0
<strong>Output:</strong> 20
<strong>Explanation:</strong> The optimal subarray is (0, 4) with a score of min(5,5,4,5,4) * (4-0+1) = 4 * 5 = 20.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 2 * 10<sup>4</sup></code>
* `0 <= k < nums.length`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let mut i = k;
        let mut j = k;
        let mut min_num = nums[k as usize];
        let mut ret = min_num;

        loop {
            while i - 1 >= 0 && nums[i as usize - 1] >= min_num {
                i -= 1;
            }
            while j + 1 < nums.len() as i32 && nums[j as usize + 1] >= min_num {
                j += 1;
            }

            ret = ret.max(min_num * (j - i + 1));
            i -= 1;
            if i < 0 {
                break;
            }
            min_num = nums[i as usize];
        }

        i = k;
        j = k;
        min_num = nums[k as usize];

        loop {
            while i - 1 >= 0 && nums[i as usize - 1] >= min_num {
                i -= 1;
            }
            while j + 1 < nums.len() as i32 && nums[j as usize + 1] >= min_num {
                j += 1;
            }

            ret = ret.max(min_num * (j - i + 1));
            j += 1;
            if j >= nums.len() as i32 {
                break;
            }
            min_num = nums[j as usize];
        }

        ret
    }
}
```
