# 775. Global and Local Inversions
You are given an integer array `nums` of length `n` which represents a permutation of all the integers in the range `[0, n - 1]`.

The number of **global inversions** is the number of the different pairs `(i, j)` where:
* `0 <= i < j < n`
* `nums[i] > nums[j]`

The number of **local inversions** is the number of indices `i` where:
* `0 <= i < n - 1`
* `nums[i] > nums[i + 1]`

Return `true` *if the number of **global inversions** is equal to the number of **local inversions***.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,0,2]
<strong>Output:</strong> true
<strong>Explanation:</strong> There is 1 global inversion and 1 local inversion.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,0]
<strong>Output:</strong> false
<strong>Explanation:</strong> There are 2 global inversions and 1 local inversion.
</pre>

#### Constraints:
* `n == nums.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* `0 <= nums[i] < n`
* All the integers of `nums` are **unique**.
* `nums` is a permutation of all the numbers in the range `[0, n - 1]`.

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[]} nums
# @return {Boolean}
def is_ideal_permutation(nums)
  max = 0

  (2...nums.size).each do |i|
    max = [max, nums[i - 2]].max
    return false if nums[i] < max
  end

  true
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        let mut max = 0;

        for i in 2..nums.len() {
            max = max.max(nums[i - 2]);
            if nums[i] < max {
                return false;
            }
        }

        true
    }
}
```
