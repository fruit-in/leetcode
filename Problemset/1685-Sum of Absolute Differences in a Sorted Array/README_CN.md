# 1685. 有序数组中差绝对值之和
给你一个 **非递减** 有序整数数组 `nums` 。

请你建立并返回一个整数数组 `result`，它跟 `nums` 长度相同，且`result[i]` 等于 `nums[i]` 与数组中所有其他元素差的绝对值之和。

换句话说， `result[i]` 等于 `sum(|nums[i]-nums[j]|)` ，其中 `0 <= j < nums.length` 且 `j != i` （下标从 0 开始）。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,3,5]
<strong>输出:</strong> [4,3,5]
<strong>解释:</strong> 假设数组下标从 0 开始，那么
result[0] = |2-2| + |2-3| + |2-5| = 0 + 1 + 3 = 4，
result[1] = |3-2| + |3-3| + |3-5| = 1 + 0 + 2 = 3，
result[2] = |5-2| + |5-3| + |5-5| = 3 + 2 + 0 = 5。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,4,6,8,10]
<strong>输出:</strong> [24,15,13,15,21]
</pre>

#### 提示:
* <code>2 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= nums[i + 1] <= 10<sup>4</sup></code>

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} nums
# @return {Integer[]}
def get_sum_absolute_differences(nums)
  total_sum = nums.sum
  left_sum = 0
  result = [0] * nums.size

  (0...nums.size).each do |i|
    result[i] = (2 * i - nums.size) * nums[i] + total_sum - 2 * left_sum
    left_sum += nums[i]
  end

  result
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len() as i32;
        let total_sum = nums.iter().sum::<i32>();
        let mut left_sum = 0;
        let mut result = vec![0; nums.len()];

        for i in 0..nums.len() {
            result[i] = (2 * i as i32 - len) * nums[i] + total_sum - 2 * left_sum;
            left_sum += nums[i];
        }

        result
    }
}
```
