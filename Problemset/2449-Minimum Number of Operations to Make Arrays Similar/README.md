# 2449. Minimum Number of Operations to Make Arrays Similar
You are given two positive integer arrays `nums` and `target`, of the same length.

In one operation, you can choose any two **distinct** indices `i` and `j` where `0 <= i, j < nums.length` and:
* set `nums[i] = nums[i] + 2` and
* set `nums[j] = nums[j] - 2`.

Two arrays are considered to be **similar** if the frequency of each element is the same.

Return *the minimum number of operations required to make* `nums` *similar to* `target`. The test cases are generated such that `nums` can always be similar to `target`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [8,12,6], target = [2,14,10]
<strong>Output:</strong> 2
<strong>Explanation:</strong> It is possible to make nums similar to target in two operations:
- Choose i = 0 and j = 2, nums = [10,12,4].
- Choose i = 1 and j = 2, nums = [10,14,2].
It can be shown that 2 is the minimum number of operations needed.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,5], target = [4,1,3]
<strong>Output:</strong> 1
<strong>Explanation:</strong> We can make nums similar to target in one operation:
- Choose i = 1 and j = 2, nums = [1,4,3].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,1,1,1,1], target = [1,1,1,1,1]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The array nums is already similiar to target.
</pre>

#### Constraints:
* `n == nums.length == target.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= nums[i], target[i] <= 10<sup>6</sup></code>
* It is possible to make `nums` similar to `target`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn make_similar(mut nums: Vec<i32>, mut target: Vec<i32>) -> i64 {
        nums.sort_unstable_by_key(|&x| (x % 2, x));
        target.sort_unstable_by_key(|&x| (x % 2, x));

        nums.iter()
            .zip(target.iter())
            .map(|(&x, &y)| (y - x).max(0) as i64 / 2)
            .sum()
    }
}
```
