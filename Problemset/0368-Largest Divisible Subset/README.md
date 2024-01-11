# 368. Largest Divisible Subset
Given a set of **distinct** positive integers `nums`, return the largest subset `answer` such that every pair `(answer[i], answer[j])` of elements in this subset satisfies:

* `answer[i] % answer[j] == 0`, or
* `answer[j] % answer[i] == 0`

If there are multiple solutions, return any of them.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3]
<strong>Output:</strong> [1,2]
<strong>Explanation:</strong> [1,3] is also accepted.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,4,8]
<strong>Output:</strong> [1,2,4,8]
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* <code>1 <= nums[i] <= 2 * 10<sup>9</sup></code>
* All the integers in `nums` are **unique**.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut max_len = vec![1; nums.len()];
        let mut prev_index = vec![None; nums.len()];
        let mut answer = vec![];

        nums.sort_unstable();

        for i in 1..nums.len() {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && max_len[i] < max_len[j] + 1 {
                    max_len[i] = max_len[j] + 1;
                    prev_index[i] = Some(j);
                }
            }
        }

        let mut curr = (0..nums.len()).max_by_key(|&i| max_len[i]).unwrap();
        answer.push(nums[curr]);

        while let Some(i) = prev_index[curr] {
            curr = i;
            answer.push(nums[curr]);
        }

        answer
    }
}
```
