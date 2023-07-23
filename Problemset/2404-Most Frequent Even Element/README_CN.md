# 2404. 出现最频繁的偶数元素
给你一个整数数组 `nums` ，返回出现最频繁的偶数元素。

如果存在多个满足条件的元素，只需要返回 **最小** 的一个。如果不存在这样的元素，返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [0,1,2,2,4,4,1]
<strong>输出:</strong> 2
<strong>解释:</strong>
数组中的偶数元素为 0、2 和 4 ，在这些元素中，2 和 4 出现次数最多。
返回最小的那个，即返回 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [4,4,4,9,2,4]
<strong>输出:</strong> 4
<strong>解释:</strong> 4 是出现最频繁的偶数元素。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [29,47,21,41,13,37,25,7]
<strong>输出:</strong> -1
<strong>解释:</strong> 不存在偶数元素。
</pre>

#### 提示:
* `1 <= nums.length <= 2000`
* <code>0 <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut max_count = 0;
        let mut ret = -1;

        for num in nums.iter().filter(|&&x| x % 2 == 0) {
            count.entry(num).and_modify(|x| *x += 1).or_insert(1);
        }

        for (&k, v) in count {
            if v > max_count || (v == max_count && k < ret) {
                max_count = v;
                ret = k;
            }
        }

        ret
    }
}
```
