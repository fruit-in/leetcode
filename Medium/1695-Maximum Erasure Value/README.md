# 1695. Maximum Erasure Value
You are given an array of positive integers `nums` and want to erase a subarray containing **unique elements**. The **score** you get by erasing the subarray is equal to the **sum** of its elements.

Return *the **maximum score** you can get by erasing **exactly one** subarray*.

An array `b` is called to be a subarray of `a` if it forms a contiguous subsequence of `a`, that is, if it is equal to `a[l],a[l+1],...,a[r]` for some `(l,r)`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [4,2,4,5,6]
<strong>Output:</strong> 17
<strong>Explanation:</strong> The optimal subarray here is [2,4,5,6].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [5,2,1,2,5,2,1,2,5]
<strong>Output:</strong> 8
<strong>Explanation:</strong> The optimal subarray here is [5,2,1] or [1,2,5].
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>4</sup></code>

## Solutions (Ruby)

### 1. Two Pointers
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def maximum_unique_subarray(nums)
  i = 0
  counter = {}
  counter.default = 0
  sum = 0
  ret = 0

  (0...nums.size).each do |j|
    counter[nums[j]] += 1
    sum += nums[j]
    while counter[nums[j]] > 1
      counter[nums[i]] -= 1
      sum -= nums[i]
      i += 1
    end
    ret = [ret, sum].max
  end

  ret
end
```

## Solutions (Rust)

### 1. Two Pointers
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut counter = HashMap::new();
        let mut sum = 0;
        let mut ret = 0;

        for j in 0..nums.len() {
            *counter.entry(nums[j]).or_insert(0) += 1;
            sum += nums[j];
            while *counter.get(&nums[j]).unwrap() > 1 {
                *counter.get_mut(&nums[i]).unwrap() -= 1;
                sum -= nums[i];
                i += 1;
            }
            ret = ret.max(sum);
        }

        ret
    }
}
```
