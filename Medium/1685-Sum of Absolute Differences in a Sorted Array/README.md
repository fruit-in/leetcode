# 1685. Sum of Absolute Differences in a Sorted Array
You are given an integer array `nums` sorted in **non-decreasing** order.

Build and return *an integer array* `result` *with the same length as* `nums` *such that* `result[i]` *is equal to the **summation of absolute differences** between* `nums[i]` *and all the other elements in the array*.

In other words, `result[i]` is equal to `sum(|nums[i]-nums[j]|)` where `0 <= j < nums.length` and `j != i` (**0-indexed**).

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,3,5]
<strong>Output:</strong> [4,3,5]
<strong>Explanation:</strong> Assuming the arrays are 0-indexed, then
result[0] = |2-2| + |2-3| + |2-5| = 0 + 1 + 3 = 4,
result[1] = |3-2| + |3-3| + |3-5| = 1 + 0 + 2 = 3,
result[2] = |5-2| + |5-3| + |5-5| = 3 + 2 + 0 = 5.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,4,6,8,10]
<strong>Output:</strong> [24,15,13,15,21]
</pre>

#### Constraints:
* <code>2 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= nums[i + 1] <= 10<sup>4</sup></code>

## Solutions (Ruby)

### 1. Solution
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

## Solutions (Rust)

### 1. Solution
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
