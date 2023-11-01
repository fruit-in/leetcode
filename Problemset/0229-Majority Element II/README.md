# 229. Majority Element II
Given an integer array of size `n`, find all elements that appear more than `⌊ n/3 ⌋` times.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,2,3]
<strong>Output:</strong> [3]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1]
<strong>Output:</strong> [1]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,2]
<strong>Output:</strong> [1,2]
</pre>

#### Constraints:
* <code>1 <= nums.length <= 5 * 10<sup>4</sup></code>
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>

**Follow up:** Could you solve the problem in linear time and in `O(1)` space?

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut majorities = [(i32::MAX, 0); 2];

        for num in &nums {
            if let Some(i) = majorities.iter().position(|(x, _)| x == num) {
                majorities[i].1 += 1;
            } else if let Some(i) = majorities.iter().position(|&(_, c)| c == 0) {
                majorities[i] = (*num, 1);
            } else {
                for i in 0..2 {
                    majorities[i].1 -= 1;
                }
            }
        }

        for i in 0..2 {
            majorities[i].1 = 0;
        }

        for num in &nums {
            if let Some(i) = majorities.iter().position(|(x, _)| x == num) {
                majorities[i].1 += 1;
            }
        }

        majorities
            .into_iter()
            .filter(|&(x, c)| *c > n / 3)
            .map(|(x, _)| *x)
            .collect()
    }
}
```
