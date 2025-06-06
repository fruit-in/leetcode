# 2587. Rearrange Array to Maximize Prefix Score
You are given a **0-indexed** integer array `nums`. You can rearrange the elements of `nums` to **any order** (including the given order).

Let `prefix` be the array containing the prefix sums of `nums` after rearranging it. In other words, `prefix[i]` is the sum of the elements from `0` to `i` in `nums` after rearranging it. The **score** of `nums` is the number of positive integers in the array `prefix`.

Return *the maximum score you can achieve*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,-1,0,1,-3,3,-3]
<strong>Output:</strong> 6
<strong>Explanation:</strong> We can rearrange the array into nums = [2,3,1,-1,-3,0,-3].
prefix = [2,5,6,5,2,2,-1], so the score is 6.
It can be shown that 6 is the maximum score we can obtain.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [-2,-3,0]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Any rearrangement of the array will result in a score of 0.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>6</sup> <= nums[i] <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_score(mut nums: Vec<i32>) -> i32 {
        let mut prefix = vec![0; nums.len() + 1];

        nums.sort_unstable_by_key(|x| -x);

        for i in 0..nums.len() {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }

        prefix.iter().filter(|&&x| x > 0).count() as i32
    }
}
```
