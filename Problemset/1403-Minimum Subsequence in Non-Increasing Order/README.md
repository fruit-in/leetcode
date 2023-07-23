# 1403. Minimum Subsequence in Non-Increasing Order
Given the array `nums`, obtain a subsequence of the array whose sum of elements is **strictly greater** than the sum of the non included elements in such subsequence.

If there are multiple solutions, return the subsequence with **minimum size** and if there still exist multiple solutions, return the subsequence with the **maximum total sum** of all its elements. A subsequence of an array can be obtained by erasing some (possibly zero) elements from the array.

Note that the solution with the given constraints is guaranteed to be **unique**. Also return the answer sorted in **non-increasing** order.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [4,3,10,9,8]
<strong>Output:</strong> [10,9]
<strong>Explanation:</strong> The subsequences [10,9] and [10,8] are minimal such that the sum of their elements is strictly greater than the sum of elements not included, however, the subsequence [10,9] has the maximum total sum of its elements. 
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [4,4,7,6,7]
<strong>Output:</strong> [7,7,6]
<strong>Explanation:</strong> The subsequence [7,7] has the sum of its elements equal to 14 which is not strictly greater than the sum of elements not included (14 = 4 + 4 + 6). Therefore, the subsequence [7,6,7] is the minimal satisfying the conditions. Note the subsequence has to returned in non-decreasing order.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [6]
<strong>Output:</strong> [6]
</pre>

#### Constraints:
* `1 <= nums.length <= 500`
* `1 <= nums[i] <= 100`

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let half = nums.iter().sum::<i32>() / 2;
        let mut acc = 0;
        nums.sort_unstable_by(|a, b| b.cmp(a));

        nums.into_iter()
            .take_while(|x| {
                acc += x;
                acc - x <= half
            })
            .collect()
    }
}
```
