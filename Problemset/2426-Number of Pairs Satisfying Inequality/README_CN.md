# 2426. 满足不等式的数对数目
给你两个下标从 **0** 开始的整数数组 `nums1` 和 `nums2` ，两个数组的大小都为 `n` ，同时给你一个整数 `diff` ，统计满足以下条件的 **数对** `(i, j)` ：

* `0 <= i < j <= n - 1` **且**
* `nums1[i] - nums1[j] <= nums2[i] - nums2[j] + diff`.

请你返回满足条件的 **数对数目** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [3,2,5], nums2 = [2,2,1], diff = 1
<strong>输出:</strong> 3
<strong>解释:</strong>
总共有 3 个满足条件的数对：
1. i = 0, j = 1：3 - 2 <= 2 - 2 + 1 。因为 i < j 且 1 <= 1 ，这个数对满足条件。
2. i = 0, j = 2：3 - 5 <= 2 - 1 + 1 。因为 i < j 且 -2 <= 2 ，这个数对满足条件。
3. i = 1, j = 2：2 - 5 <= 2 - 1 + 1 。因为 i < j 且 -3 <= 2 ，这个数对满足条件。
所以，我们返回 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [3,-1], nums2 = [-2,2], diff = -1
<strong>输出:</strong> 0
<strong>解释:</strong>
没有满足条件的任何数对，所以我们返回 0 。
</pre>

#### 提示:
* `n == nums1.length == nums2.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= nums1[i], nums2[i] <= 10<sup>4</sup></code>
* <code>-10<sup>4</sup> <= diff <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
        let n = nums1.len();
        let nums = (0..n).map(|i| nums1[i] - nums2[i]).collect::<Vec<_>>();
        let mut nums_diff = nums.clone();
        let mut hashmap = HashMap::new();
        let mut tree = vec![0; n * 2 + 1];
        let mut ret = 0;

        for i in 0..n {
            nums_diff.push(nums[i] + diff);
        }
        nums_diff.sort_unstable();
        for i in 1..tree.len() {
            hashmap.insert(nums_diff[i - 1], i as i32);
        }

        for i in 0..n {
            let mut temp = hashmap[&(nums[i] + diff)];

            while temp > 0 {
                ret += tree[temp as usize];
                temp -= temp & (-temp);
            }

            temp = hashmap[&nums[i]];

            while temp < tree.len() as i32 {
                tree[temp as usize] += 1;
                temp += temp & (-temp);
            }
        }

        ret
    }
}
```
