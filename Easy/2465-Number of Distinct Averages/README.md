# 2465. Number of Distinct Averages
You are given a **0-indexed** integer array `nums` of **even** length.

As long as `nums` is **not** empty, you must repetitively:
* Find the minimum number in `nums` and remove it.
* Find the maximum number in `nums` and remove it.
* Calculate the average of the two removed numbers.

The **average** of two numbers `a` and `b` is `(a + b) / 2`.

* For example, the average of `2` and `3` is `(2 + 3) / 2 = 2.5`.

Return *the number of **distinct** averages calculated using the above process*.

**Note** that when there is a tie for a minimum or maximum number, any can be removed.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [4,1,4,0,3,5]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
1. Remove 0 and 5, and the average is (0 + 5) / 2 = 2.5. Now, nums = [4,1,4,3].
2. Remove 1 and 4. The average is (1 + 4) / 2 = 2.5, and nums = [4,3].
3. Remove 3 and 4, and the average is (3 + 4) / 2 = 3.5.
Since there are 2 distinct numbers among 2.5, 2.5, and 3.5, we return 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,100]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
There is only one average to be calculated after removing 1 and 100, so we return 1.
</pre>

#### Constraints:
* `2 <= nums.length <= 100`
* `nums.length` is even.
* `0 <= nums[i] <= 100`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn distinct_averages(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        (0..nums.len() / 2)
            .map(|i| nums[i] + nums[nums.len() - 1 - i])
            .collect::<HashSet<_>>()
            .len() as i32
    }
}
```
