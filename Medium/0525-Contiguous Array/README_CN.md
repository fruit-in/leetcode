# 525. 连续数组
给定一个二进制数组, 找到含有相同数量的 0 和 1 的最长连续子数组（的长度）。

#### 示例 1:
<pre>
<strong>输入:</strong> [0,1]
<strong>输出:</strong> 2
<strong>说明:</strong> [0, 1] 是具有相同数量0和1的最长连续子数组。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [0,1,0]
<strong>输出:</strong> 2
<strong>说明:</strong> [0, 1] (或 [1, 0]) 是具有相同数量0和1的最长连续子数组。
</pre>

**注意:** 给定的二进制数组的长度不会超过50000。

## 题解 (Ruby)

### 1. 哈希表
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

## 题解 (Rust)

### 1. 哈希表
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
