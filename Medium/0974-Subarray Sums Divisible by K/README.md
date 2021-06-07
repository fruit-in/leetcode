# 974. Subarray Sums Divisible by K
Given an array `nums` of integers, return the number of (contiguous, non-empty) subarrays that have a sum divisible by `k`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [4,5,0,-2,-3,1], k = 5
<strong>Output:</strong> 7
<strong>Explanation:</strong> There are 7 subarrays with a sum divisible by k = 5:
[4, 5, 0, -2, -3, 1], [5], [5, 0], [5, 0, -2, -3], [0], [0, -2, -3], [-2, -3]
</pre>

#### Note:
1. `1 <= nums.length <= 30000`
2. `-10000 <= nums[i] <= 10000`
3. `2 <= k <= 10000`

## Solutions (Ruby)

### 1. Prefix Sum
```Ruby
# @param {Integer[]} a
# @param {Integer} k
# @return {Integer}
def subarrays_div_by_k(a, k)
  counter = { 0 => 1 }
  counter.default = 0
  sum = 0
  ret = 0

  a.each do |x|
    sum = (sum + x) % k
    ret += counter[sum]
    counter[sum] += 1
  end

  ret
end
```

## Solutions (Rust)

### 1. Prefix Sum
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut counter = vec![(0, 1)].into_iter().collect::<HashMap<_, _>>();
        let mut sum = 0;
        let mut ret = 0;

        for x in nums {
            sum = ((sum + x) % k + k) % k;
            let count = counter.entry(sum).or_insert(0);
            ret += *count;
            *count += 1;
        }

        ret
    }
}
```
