# 416. 分割等和子集
给你一个 **只包含正整数** 的 **非空** 数组 `nums` 。请你判断是否可以将这个数组分割成两个子集，使得两个子集的元素和相等。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,5,11,5]
<strong>输出:</strong> true
<strong>解释:</strong> 数组可以分割成 [1, 5, 5] 和 [11] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,5]
<strong>输出:</strong> false
<strong>解释:</strong> 数组不能分割成两个元素和相等的子集。
</pre>

#### 提示:
* `1 <= nums.length <= 200`
* `1 <= nums[i] <= 100`

## 题解 (Ruby)

### 1. 动态规划
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

## 题解 (Rust)

### 1. 动态规划
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
