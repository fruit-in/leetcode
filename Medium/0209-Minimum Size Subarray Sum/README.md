# 209. Minimum Size Subarray Sum
Given an array of **n** positive integers and a positive integer **s**, find the minimal length of a **contiguous** subarray of which the sum ≥ **s**. If there isn't one, return 0 instead.

#### Example:
<pre>
<strong>Input:</strong> s = 7, nums = [2,3,1,2,4,3]
<strong>Output:</strong> 2
<strong>Explanation:</strong> the subarray [4,3] has the minimal length under the problem constraint.
</pre>

#### Follow up:
If you have figured out the *O*(*n*) solution, try coding another solution of which the time complexity is *O*(*n* log *n*).

## Solutions (Rust)

### 1. Two Pointers
```Rust
impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        if nums.iter().sum::<i32>() < s {
            return 0;
        }

        let mut i = 0;
        let mut sum = 0;
        let mut ret = std::i32::MAX;

        for j in 0..nums.len() {
            sum += nums[j];
            while sum >= s {
                ret = ret.min((j - i) as i32 + 1);
                sum -= nums[i];
                i += 1;
            }
        }

        ret
    }
}
```
