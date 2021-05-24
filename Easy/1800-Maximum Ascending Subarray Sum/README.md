# 1800. Maximum Ascending Subarray Sum
Given an array of positive integers `nums`, return the *maximum possible sum of an **ascending** subarray in* `nums`.

A subarray is defined as a contiguous sequence of numbers in an array.

A subarray <code>[nums<sub>l</sub>, nums<sub>l+1</sub>, ..., nums<sub>r-1</sub>, nums<sub>r</sub>]</code> is **ascending** if for all `i` where `l <= i < r`, <code>nums<sub>i</sub> < nums<sub>i+1</sub></code>. Note that a subarray of size `1` is **ascending**.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [10,20,30,5,10,50]
<strong>Output:</strong> 65
<strong>Explanation:</strong> [5,10,50] is the ascending subarray with the maximum sum of 65.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [10,20,30,40,50]
<strong>Output:</strong> 150
<strong>Explanation:</strong> [10,20,30,40,50] is the ascending subarray with the maximum sum of 150.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [12,17,15,13,10,11,12]
<strong>Output:</strong> 33
<strong>Explanation:</strong> [10,11,12] is the ascending subarray with the maximum sum of 33.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> nums = [100,10,1]
<strong>Output:</strong> 100
</pre>

#### Constraints:
* `1 <= nums.length <= 100`
* `1 <= nums[i] <= 100`

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def max_ascending_sum(nums)
  sum = nums[0]
  ret = sum

  (1...nums.size).each do |i|
    sum = nums[i] + (nums[i] > nums[i - 1] ? sum : 0)
    ret = [ret, sum].max
  end

  ret
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut sum = nums[0];
        let mut ret = sum;

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                sum += nums[i];
            } else {
                sum = nums[i];
            }
            ret = ret.max(sum);
        }

        ret
    }
}
```
