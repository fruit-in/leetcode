# 1726. 同积元组
给你一个由 **不同** 正整数组成的数组 `nums` ，请你返回满足 `a * b = c * d` 的元组 `(a, b, c, d)` 的数量。其中 `a`、`b`、`c` 和 `d` 都是 `nums` 中的元素，且 `a != b != c != d` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,3,4,6]
<strong>输出:</strong> 8
<strong>解释:</strong> 存在 8 个满足题意的元组：
(2,6,3,4) , (2,6,4,3) , (6,2,3,4) , (6,2,4,3)
(3,4,2,6) , (4,3,2,6) , (3,4,6,2) , (4,3,6,2)
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,4,5,10]
<strong>输出:</strong> 16
<strong>解释:</strong> 存在 16 个满足题意的元组：
(1,10,2,5) , (1,10,5,2) , (10,1,2,5) , (10,1,5,2)
(2,5,1,10) , (2,5,10,1) , (5,2,1,10) , (5,2,10,1)
(2,10,4,5) , (2,10,5,4) , (10,2,4,5) , (10,2,4,5)
(4,5,2,10) , (4,5,10,2) , (5,4,2,10) , (5,4,10,2)
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [2,3,4,6,8,12]
<strong>输出:</strong> 40
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> nums = [2,3,5,7]
<strong>输出:</strong> 0
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* <code>1 <= nums[i] <= 10<sup>4</sup></code>
* `nums` 中的所有元素 **互不相同**

## 题解 (Ruby)

### 1. 哈希表
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

## 题解 (Rust)

### 1. 哈希表
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
