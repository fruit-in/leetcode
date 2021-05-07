# 1248. Count Number of Nice Subarrays
Given an array of integers `nums` and an integer `k`. A continuous subarray is called **nice** if there are `k` odd numbers on it.

Return *the number of **nice** sub-arrays*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,1,2,1,1], k = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> The only sub-arrays with 3 odd numbers are [1,1,2,1] and [1,2,1,1].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,4,6], k = 1
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no odd numbers in the array.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [2,2,2,1,2,2,1,2,2,2], k = 2
<strong>Output:</strong> 16
</pre>

#### Constraints:
* `1 <= nums.length <= 50000`
* `1 <= nums[i] <= 10^5`
* `1 <= k <= nums.length`

## Solutions (Ruby)

### 1. HashMap
```Ruby
# @param {Integer[]} nums
# @param {Integer} k
# @return {Integer}
def number_of_subarrays(nums, k)
  counter = { 0 => 1 }
  counter.default = 0
  count = 0
  ret = 0

  nums.each do |x|
    count += 1 if x.odd?
    ret += counter[count - k]
    counter[count] += 1
  end

  ret
end
```

## Solutions (Rust)

### 1. HashMap
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut counter = vec![(0, 1)].into_iter().collect::<HashMap<_, _>>();
        let mut count = 0;
        let mut ret = 0;

        for i in 0..nums.len() {
            if nums[i] % 2 == 1 {
                count += 1;
            }
            ret += counter.get(&(count - k)).unwrap_or(&0);
            *counter.entry(count).or_insert(0) += 1;
        }

        ret
    }
}
```
