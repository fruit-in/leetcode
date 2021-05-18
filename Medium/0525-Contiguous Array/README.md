# 525. Contiguous Array
Given a binary array `nums`, return *the maximum length of a contiguous subarray with an equal number of* `0` *and* `1`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [0,1]
<strong>Output:</strong> 2
<strong>Explanation:</strong> [0, 1] is the longest contiguous subarray with an equal number of 0 and 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [0,1,0]
<strong>Output:</strong> 2
<strong>Explanation:</strong> [0, 1] (or [1, 0]) is a longest contiguous subarray with equal number of 0 and 1.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* `nums[i]` is either `0` or `1`.

## Solutions (Ruby)

### 1. HashMap
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def find_max_length(nums)
  count = 0
  counts = { 0 => -1 }
  ret = 0

  (0...nums.size).each do |i|
    count += nums[i] == 0 ? 1 : -1
    counts[count] = i unless counts.include?(count)
    ret = [ret, i - counts[count]].max
  end

  ret
end
```

## Solutions (Rust)

### 1. HashMap
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut counts = vec![(0, -1)].into_iter().collect::<HashMap<_, _>>();
        let mut ret = 0;

        for i in 0..nums.len() as i32 {
            match nums[i as usize] {
                0 => count += 1,
                _ => count -= 1,
            }
            if !counts.contains_key(&count) {
                counts.insert(count, i);
            }
            ret = ret.max(i - counts.get(&count).unwrap());
        }

        ret
    }
}
```
