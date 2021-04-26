# 416. Partition Equal Subset Sum
Given a **non-empty** array `nums` containing **only positive integers**, find if the array can be partitioned into two subsets such that the sum of elements in both subsets is equal.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,5,11,5]
<strong>Output:</strong> true
<strong>Explanation:</strong> The array can be partitioned as [1, 5, 5] and [11].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,5]
<strong>Output:</strong> false
<strong>Explanation:</strong> The array cannot be partitioned into equal sum subsets.
</pre>

#### Constraints:
* `1 <= nums.length <= 200`
* `1 <= nums[i] <= 100`

## Solutions (Ruby)

### 1. Dynamic Programming
```Ruby
# @param {Integer[]} nums
# @return {Boolean}
def can_partition(nums)
  sum = nums.sum
  target = sum / 2
  return false if sum.odd? || nums.any? { |x| x > target }

  dp = [false] * (target + 1)
  dp[0] = true

  nums.each { |x| (0..target - x).reverse_each { |i| dp[i + x] |= dp[i] } }

  dp[target]
end
```

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>() as usize;
        let target = sum / 2;
        if sum % 2 == 1 || nums.iter().any(|&x| x as usize > target) {
            return false;
        }
        let mut dp = vec![false; target + 1];
        dp[0] = true;

        for x in nums {
            for i in (0..=target - x as usize).rev() {
                dp[i + x as usize] |= dp[i];
            }
        }

        dp[target]
    }
}
```
