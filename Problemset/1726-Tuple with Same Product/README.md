# 1726. Tuple with Same Product
Given an array `nums` of **distinct** positive integers, return *the number of tuples* `(a, b, c, d)` *such that* `a * b = c * d` *where* `a`, `b`, `c`, *and* `d` *are elements of* `nums`, *and* `a != b != c != d`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,3,4,6]
<strong>Output:</strong> 8
<strong>Explanation:</strong> There are 8 valid tuples:
(2,6,3,4) , (2,6,4,3) , (6,2,3,4) , (6,2,4,3)
(3,4,2,6) , (4,3,2,6) , (3,4,6,2) , (4,3,6,2)
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,4,5,10]
<strong>Output:</strong> 16
<strong>Explanation:</strong> There are 16 valids tuples:
(1,10,2,5) , (1,10,5,2) , (10,1,2,5) , (10,1,5,2)
(2,5,1,10) , (2,5,10,1) , (5,2,1,10) , (5,2,10,1)
(2,10,4,5) , (2,10,5,4) , (10,2,4,5) , (10,2,4,5)
(4,5,2,10) , (4,5,10,2) , (5,4,2,10) , (5,4,10,2)
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [2,3,4,6,8,12]
<strong>Output:</strong> 40
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> nums = [2,3,5,7]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* <code>1 <= nums[i] <= 10<sup>4</sup></code>
* All elements in `nums` are **distinct**.

## Solutions (Ruby)

### 1. HashMap
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def tuple_same_product(nums)
  counter = {}
  counter.default = 0

  (0...nums.size).each do |i|
    (i + 1...nums.size).each do |j|
      counter[nums[i] * nums[j]] += 1
    end
  end

  counter.values.map { |c| c * (c - 1) * 4 }.sum
end
```

## Solutions (Rust)

### 1. HashMap
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                *counter.entry(nums[i] * nums[j]).or_insert(0) += 1;
            }
        }

        counter.values().map(|c| c * (c - 1) * 4).sum()
    }
}
```
