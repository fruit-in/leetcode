# 1752. Check if Array Is Sorted and Rotated
Given an array `nums`, return `true` *if the array was originally sorted in non-decreasing order, then rotated **some** number of positions (including zero)*. Otherwise, return `false`.

There may be **duplicates** in the original array.

**Note:** An array `A` rotated by `x` positions results in an array `B` of the same length such that `A[i] == B[(i+x) % A.length]`, where `%` is the modulo operation.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,4,5,1,2]
<strong>Output:</strong> true
<strong>Explanation:</strong> [1,2,3,4,5] is the original sorted array.
You can rotate the array by x = 3 positions to begin on the the element of value 3: [3,4,5,1,2].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,1,3,4]
<strong>Output:</strong> false
<strong>Explanation:</strong> There is no sorted array once rotated that can make nums.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,2,3]
<strong>Output:</strong> true
<strong>Explanation:</strong> [1,2,3] is the original sorted array.
You can rotate the array by x = 0 positions (i.e. no rotation) to make nums.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> nums = [1,1,1]
<strong>Output:</strong> true
<strong>Explanation:</strong> [1,1,1] is the original sorted array.
You can rotate any number of positions to make nums.
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> nums = [2,1]
<strong>Output:</strong> true
<strong>Explanation:</strong> [1,2] is the original sorted array.
You can rotate the array by x = 5 positions to begin on the element of value 2: [2,1].
</pre>

#### Constraints:
* `1 <= nums.length <= 100`
* `1 <= nums[i] <= 100`

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[]} nums
# @return {Boolean}
def check(nums)
  count = (1...nums.size).count { |i| nums[i] < nums[i - 1] }
  count += 1 if nums[0] < nums[-1]

  count < 2
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        match (1..nums.len()).filter(|&i| nums[i] < nums[i - 1]).count() {
            0 => true,
            1 => nums[0] >= nums[nums.len() - 1],
            _ => false,
        }
    }
}
```
