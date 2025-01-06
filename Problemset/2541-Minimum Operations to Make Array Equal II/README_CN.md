# 2541. 使数组中所有元素相等的最小操作数 II
给你两个整数数组 `nums1` 和 `nums2` ，两个数组长度都是 `n` ，再给你一个整数 `k` 。你可以对数组 `nums1` 进行以下操作：

* 选择两个下标 `i` 和 `j` ，将 `nums1[i]` 增加 `k` ，将 `nums1[j]` 减少 `k` 。换言之，`nums1[i] = nums1[i] + k` 且 `nums1[j] = nums1[j] - k` 。

如果对于所有满足 `0 <= i < n` 都有 `num1[i] == nums2[i]` ，那么我们称 `nums1` **等于** `nums2` 。

请你返回使 `nums1` 等于 `nums2` 的 **最少** 操作数。如果没办法让它们相等，请你返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [4,3,1,4], nums2 = [1,3,7,1], k = 3
<strong>输出:</strong> 2
<strong>解释:</strong> 我们可以通过 2 个操作将 nums1 变成 nums2 。
第 1 个操作：i = 2 ，j = 0 。操作后得到 nums1 = [1,3,4,4] 。
第 2 个操作：i = 2 ，j = 3 。操作后得到 nums1 = [1,3,7,1] 。
无法用更少操作使两个数组相等。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [3,8,5,2], nums2 = [2,4,1,6], k = 1
<strong>输出:</strong> -1
<strong>解释:</strong> 无法使两个数组相等。
</pre>

#### 提示:
* `n == nums1.length == nums2.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>0 <= nums1[i], nums2[j] <= 10<sup>9</sup></code>
* <code>0 <= k <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        if k == 0 {
            if (0..nums1.len()).all(|i| nums1[i] == nums2[i]) {
                return 0;
            } else {
                return -1;
            }
        }

        let mut inc = 0;
        let mut dec = 0;

        for i in 0..nums1.len() {
            if nums1[i] < nums2[i] && (nums2[i] - nums1[i]) % k == 0 {
                inc += ((nums2[i] - nums1[i]) / k) as i64;
            } else if nums1[i] > nums2[i] && (nums1[i] - nums2[i]) % k == 0 {
                dec += ((nums1[i] - nums2[i]) / k) as i64;
            } else if nums1[i] != nums2[i] {
                return -1;
            }
        }

        if inc == dec {
            inc
        } else {
            -1
        }
    }
}
```
