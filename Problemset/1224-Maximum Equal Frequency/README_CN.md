# 1224. 最大相等频率
给你一个正整数数组 `nums`，请你帮忙从该数组中找出能满足下面要求的 **最长** 前缀，并返回该前缀的长度：

* 从前缀中 **恰好删除一个** 元素后，剩下每个数字的出现次数都相同。

如果删除这个元素后没有剩余元素存在，仍可认为每个数字都具有相同的出现次数（也就是 0 次）。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,2,1,1,5,3,3,5]
<strong>输出:</strong> 7
<strong>解释:</strong> 对于长度为 7 的子数组 [2,2,1,1,5,3,3]，如果我们从中删去 nums[4] = 5，就可以得到 [2,2,1,1,3,3]，里面每个数字都出现了两次。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,1,1,2,2,2,3,3,3,4,4,4,5]
<strong>输出:</strong> 13
</pre>

#### 提示:
* <code>2 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut count_nums = HashMap::new();
        let mut count_ocurrences = HashMap::new();

        for &num in &nums {
            *count_nums.entry(num).or_insert(0) += 1;
        }
        for &ocurrence in count_nums.values() {
            *count_ocurrences.entry(ocurrence).or_insert(0) += 1;
        }

        for i in (0..nums.len()).rev() {
            if count_ocurrences.len() == 1 {
                let (&k0, &v0) = count_ocurrences.iter().next().unwrap();

                if k0 == 1 || v0 == 1 {
                    return i as i32 + 1;
                }
            } else if count_ocurrences.len() == 2 {
                let (&k0, &v0) = count_ocurrences.iter().max().unwrap();
                let (&k1, &v1) = count_ocurrences.iter().min().unwrap();

                if (k1, v1) == (1, 1) || (k0 == k1 + 1 && v0 == 1) {
                    return i as i32 + 1;
                }
            }

            *count_ocurrences.get_mut(&count_nums[&nums[i]]).unwrap() -= 1;
            if count_ocurrences.get(&(count_nums[&nums[i]])) == Some(&0) {
                count_ocurrences.remove(&count_nums[&nums[i]]);
            }
            *count_nums.get_mut(&nums[i]).unwrap() -= 1;
            if count_nums[&nums[i]] > 0 {
                *count_ocurrences.entry(count_nums[&nums[i]]).or_insert(0) += 1;
            }
        }

        1
    }
}
```
