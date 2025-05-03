# 2333. 最小差值平方和
给你两个下标从 **0** 开始的整数数组 `nums1` 和 `nums2` ，长度为 `n` 。

数组 `nums1` 和 `nums2` 的 **差值平方和** 定义为所有满足 `0 <= i < n` 的 <code>(nums1[i] - nums2[i])<sup>2</sup></code> 之和。

同时给你两个正整数 `k1` 和 `k2` 。你可以将 `nums1` 中的任意元素 `+1` 或者 `-1` 至多 `k1` 次。类似的，你可以将 `nums2` 中的任意元素 `+1` 或者 `-1` 至多 `k2` 次。

请你返回修改数组 `nums1` 至多 `k1` 次且修改数组 `nums2` 至多 `k2` 次后的最小 **差值平方和** 。

**注意：**你可以将数组中的元素变成 **负** 整数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [1,2,3,4], nums2 = [2,10,20,19], k1 = 0, k2 = 0
<strong>输出:</strong> 579
<strong>解释:</strong> nums1 和 nums2 中的元素不能修改，因为 k1 = 0 和 k2 = 0 。
差值平方和为：(1 - 2)2 + (2 - 10)2 + (3 - 20)2 + (4 - 19)2 = 579 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [1,4,10,12], nums2 = [5,8,6,9], k1 = 1, k2 = 1
<strong>输出:</strong> 43
<strong>解释:</strong> 一种得到最小差值平方和的方式为：
- 将 nums1[0] 增加一次。
- 将 nums2[2] 增加一次。
最小差值平方和为：
(2 - 5)2 + (4 - 8)2 + (10 - 7)2 + (12 - 9)2 = 43 。
注意，也有其他方式可以得到最小差值平方和，但没有得到比 43 更小答案的方案。
</pre>

#### 提示:
* `n == nums1.length == nums2.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>0 <= nums1[i], nums2[i] <= 10<sup>5</sup></code>
* <code>0 <= k1, k2 <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let mut k = (k1 + k2) as i64;
        let mut diffs = (0..nums1.len())
            .map(|i| (nums1[i] - nums2[i]).abs() as i64)
            .collect::<Vec<_>>();
        let mut count = 1;

        diffs.push(0);
        diffs.sort_unstable();

        for i in (0..diffs.len()).rev() {
            if diffs[i] == 0 {
                return 0;
            }

            if (diffs[i] - diffs[i - 1]) * count <= k {
                k -= (diffs[i] - diffs[i - 1]) * count;
                count += 1;
            } else {
                let tmp = diffs[i] - k / count;
                k -= (diffs[i] - tmp) * count;
                return count * tmp * tmp - 2 * k * tmp
                    + k
                    + (0..i).map(|j| diffs[j] * diffs[j]).sum::<i64>();
            }
        }

        unreachable!()
    }
}
```
