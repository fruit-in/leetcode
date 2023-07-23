# 334. Increasing Triplet Subsequence
Given an integer array `nums`, return `true` *if there exists a triple of indices* `(i, j, k)` *such that* `i < j < k` *and* `nums[i] < nums[j] < nums[k]`. If no such indices exists, return `false`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,5]
<strong>Output:</strong> true
<strong>Explanation:</strong> Any triplet where i < j < k is valid.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [5,4,3,2,1]
<strong>Output:</strong> false
<strong>Explanation:</strong> No triplet exists.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [2,1,5,0,4,6]
<strong>Output:</strong> true
<strong>Explanation:</strong> The triplet (3, 4, 5) is valid because nums[3] == 0 < nums[4] == 4 < nums[5] == 6.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1</code>

**Follow up:** Could you implement a solution that runs in `O(n)` time complexity and `O(1)` space complexity?

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[]} nums
# @return {Boolean}
def increasing_triplet(nums)
  min3 = [nil, nil, nil]

  nums.each do |x|
    if min3[1].nil?
      min3[1] = x
    elsif min3[2].nil? && x <= min3[1]
      min3[1] = x
    elsif min3[2].nil?
      min3[2] = x
    elsif x > min3[2]
      return true
    elsif x > min3[1]
      min3[2] = x
    elsif min3[0].nil? && x < min3[1]
      min3[0] = x
    elsif !min3[0].nil? && x > min3[0]
      min3 = [nil, min3[0], x]
    elsif !min3[0].nil? && x < min3[0]
      min3[0] = x
    end
  end

  false
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut min3 = (None, None, None);

        for x in nums {
            match min3 {
                (_, None, _) => min3.1 = Some(x),
                (_, Some(b), None) if x < b => min3.1 = Some(x),
                (_, Some(b), None) if x > b => min3.2 = Some(x),
                (_, Some(b), Some(c)) if x > c => return true,
                (_, Some(b), Some(c)) if x > b => min3.2 = Some(x),
                (None, Some(b), Some(c)) if x < b => min3.0 = Some(x),
                (Some(a), Some(b), Some(c)) if x > a => min3 = (None, Some(a), Some(x)),
                (Some(a), Some(b), Some(c)) if x < a => min3.0 = Some(x),
                _ => (),
            }
        }

        false
    }
}
```
