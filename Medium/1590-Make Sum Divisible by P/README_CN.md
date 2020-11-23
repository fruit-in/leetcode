# 1590. 使数组和能被 P 整除
给你一个正整数数组 `nums`，请你移除 **最短** 子数组（可以为 **空**），使得剩余元素的 **和** 能被 `p` 整除。 **不允许** 将整个数组都移除。

请你返回你需要移除的最短子数组的长度，如果无法满足题目要求，返回 `-1` 。

**子数组** 定义为原数组中连续的一组元素。

#### 示例 1:
<pre>
<b>输入:</b> nums = [3,1,4,2], p = 6
<b>输出:</b> 1
<b>解释:</b> nums 中元素和为 10，不能被 p 整除。我们可以移除子数组 [4] ，剩余元素的和为 6 。
</pre>

#### 示例 2:
<pre>
<b>输入:</b> nums = [6,3,5,2], p = 9
<b>输出:</b> 2
<b>解释:</b> 我们无法移除任何一个元素使得和被 9 整除，最优方案是移除子数组 [5,2] ，剩余元素为 [6,3]，和为 9 。
</pre>

#### 示例 3:
<pre>
<b>输入:</b> nums = [1,2,3], p = 3
<b>输出:</b> 0
<b>解释:</b> 和恰好为 6 ，已经能被 3 整除了。所以我们不需要移除任何元素。
</pre>

#### 示例 4:
<pre>
<b>输入:</b> nums = [1,2,3], p = 7
<b>输出:</b> -1
<b>解释:</b> 没有任何方案使得移除子数组后剩余元素的和被 7 整除。
</pre>

#### 示例 5:
<pre>
<b>输入:</b> nums = [1000000000,1000000000,1000000000], p = 3
<b>输出:</b> 0
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>
* <code>1 <= p <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 前缀和
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let p = p as i64;
        let mut prefix = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let mut hash = HashMap::new();
        let mut ret = std::i32::MAX;

        for i in 1..prefix.len() {
            prefix[i] += prefix[i - 1];
        }

        let sum = *prefix.last().unwrap();

        for i in 0..prefix.len() {
            hash.insert(prefix[i] % p, i);
            if prefix[i] % p == 0 {
                ret = ret.min((prefix.len() - 1 - i) as i32);
            }
            if (sum - prefix[i]) % p == 0 {
                match hash.get(&0) {
                    Some(&j) => ret = ret.min((i - j) as i32),
                    None => ret = ret.min(i as i32 + 1),
                }
            }
            if let Some(&j) = hash.get(&(p - (sum - prefix[i]) % p)) {
                ret = ret.min((i - j) as i32);
            }
        }

        if ret == prefix.len() as i32 {
            return -1;
        }

        ret
    }
}
```
