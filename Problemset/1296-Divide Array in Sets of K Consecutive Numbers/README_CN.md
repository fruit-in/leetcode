# 1296. 划分数组为连续数字的集合
给你一个整数数组 `nums` 和一个正整数 `k`，请你判断是否可以把这个数组划分成一些由 `k` 个连续数字组成的集合。

如果可以，请返回 `True`；否则，返回 `False`。

**注意:** 此题目与 846 重复：https://leetcode-cn.com/problems/hand-of-straights/

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,3,4,4,5,6], k = 4
<strong>输出:</strong> true
<strong>解释:</strong> 数组可以分成 [1,2,3,4] 和 [3,4,5,6]。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [3,2,1,2,3,4,3,4,5,9,10,11], k = 3
<strong>输出:</strong> true
<strong>解释:</strong> 数组可以分成 [1,2,3] , [2,3,4] , [3,4,5] 和 [9,10,11]。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [3,3,2,2,1,1], k = 3
<strong>输出:</strong> true
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> nums = [1,2,3,4], k = 3
<strong>输出:</strong> false
<strong>解释:</strong> 数组不能分成几个大小为 3 的子数组。
</pre>

#### 提示:
* `1 <= nums.length <= 10^5`
* `1 <= nums[i] <= 10^9`
* `1 <= k <= nums.length`

## 题解 (Rust)

### 1. 哈希表
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn is_possible_divide(mut nums: Vec<i32>, k: i32) -> bool {
        let mut needs: HashMap<i32, Vec<i32>> = HashMap::new();
        nums.sort_unstable();

        for x in nums {
            if let Some(v) = needs.get_mut(&(x - 1)) {
                match v.pop() {
                    Some(1) => (),
                    Some(y) => needs.entry(x).or_insert(vec![]).push(y - 1),
                    None => needs.entry(x).or_insert(vec![]).push(k - 1),
                }
            } else if k > 1 {
                needs.entry(x).or_insert(vec![]).push(k - 1);
            }
        }

        needs.values().all(|v| v.is_empty())
    }
}
```
