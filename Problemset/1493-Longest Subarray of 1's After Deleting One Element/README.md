# 1493. Longest Subarray of 1's After Deleting One Element
Given a binary array `nums`, you should delete one element from it.

Return the size of the longest non-empty subarray containing only 1's in the resulting array.

Return 0 if there is no such subarray.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,1,0,1]
<strong>Output:</strong> 3
<strong>Explanation:</strong> After deleting the number in position 2, [1,1,1] contains 3 numbers with value of 1's.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [0,1,1,1,0,1,1,0,1]
<strong>Output:</strong> 5
<strong>Explanation:</strong> After deleting the number in position 4, [0,1,1,1,1,1,0,1] longest subarray with value of 1's is [1,1,1,1,1].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,1,1]
<strong>Output:</strong> 2
<strong>Explanation:</strong> You must delete one element.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> nums = [1,1,0,0,1,1,1,0,1]
<strong>Output:</strong> 4
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> nums = [0,0,0]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `1 <= nums.length <= 10^5`
* `nums[i]` is either `0` or `1`.

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def longest_subarray(nums)
  zeros = 0
  l = 0
  ret = 0

  (0...nums.size).each do |r|
    zeros += 1 if nums[r] == 0
    while zeros > 1
      zeros -= 1 if nums[l] == 0
      l += 1
    end
    ret = [ret, r - l - zeros + 1].max
  end

  [ret, nums.size - 1].min
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut zeros = 0;
        let mut l = 0;
        let mut ret = 0;

        for r in 0..nums.len() {
            if nums[r] == 0 {
                zeros += 1;
            }
            while zeros > 1 {
                if nums[l] == 0 {
                    zeros -= 1;
                }
                l += 1;
            }
            ret = ret.max(r - l - zeros + 1)
        }

        ret.min(nums.len() - 1) as i32
    }
}
```
