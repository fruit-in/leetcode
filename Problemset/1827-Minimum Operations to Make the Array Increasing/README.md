# 1827. Minimum Operations to Make the Array Increasing
You are given an integer array `nums` (**0-indexed**). In one operation, you can choose an element of the array and increment it by `1`.
* For example, if `nums = [1,2,3]`, you can choose to increment `nums[1]` to make <code>nums = [1,<b>3</b>,3]</code>.

Return *the **minimum** number of operations needed to make* `nums` ***strictly increasing***.

An array `nums` is **strictly increasing** if `nums[i] < nums[i+1]` for all `0 <= i < nums.length - 1`. An array of length `1` is trivially strictly increasing.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,1,1]
<strong>Output:</strong> 3
<strong>Explanation:</strong> You can do the following operations:
1) Increment nums[2], so nums becomes [1,1,<b>2</b>].
2) Increment nums[1], so nums becomes [1,<b>2</b>,2].
3) Increment nums[2], so nums becomes [1,2,<b>3</b>].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,5,2,4,1]
<strong>Output:</strong> 14
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [8]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `1 <= nums.length <= 5000`
* <code>1 <= nums[i] <= 10<sup>4</sup></code>

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def min_operations(nums)
  ret = 0

  (1...nums.size).each do |i|
    ret += [nums[i], nums[i - 1] + 1].max - nums[i]
    nums[i] = [nums[i], nums[i - 1] + 1].max
  end

  ret
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut ret = 0;

        for i in 1..nums.len() {
            ret += nums[i].max(nums[i - 1] + 1) - nums[i];
            nums[i] = nums[i].max(nums[i - 1] + 1);
        }

        ret
    }
}
```
