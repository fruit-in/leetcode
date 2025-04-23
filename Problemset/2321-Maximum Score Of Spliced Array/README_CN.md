# 2321. 拼接数组的最大分数
给你两个下标从 **0** 开始的整数数组 `nums1` 和 `nums2` ，长度都是 `n` 。

你可以选择两个整数 `left` 和 `right` ，其中 `0 <= left <= right < n` ，接着 **交换** 两个子数组 `nums1[left...right]` 和 `nums2[left...right]` 。

* 例如，设 `nums1 = [1,2,3,4,5]` 和 `nums2 = [11,12,13,14,15]` ，整数选择 `left = 1` 和 `right = 2`，那么 `nums1` 会变为 `[1,12,13,4,5]` 而 `nums2` 会变为 `[11,2,3,14,15]` 。

你可以选择执行上述操作 **一次** 或不执行任何操作。

数组的 **分数** 取 `sum(nums1)` 和 `sum(nums2)` 中的最大值，其中 `sum(arr)` 是数组 `arr` 中所有元素之和。

返回 **可能的最大分数** 。

**子数组** 是数组中连续的一个元素序列。`arr[left...right]` 表示子数组包含 `nums` 中下标 `left` 和 `right` 之间的元素（**含** 下标 `left` 和 `right` 对应元素）。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [60,60,60], nums2 = [10,90,10]
<strong>输出:</strong> 210
<strong>解释:</strong> 选择 left = 1 和 right = 1 ，得到 nums1 = [60,90,60] 和 nums2 = [10,60,10] 。
分数为 max(sum(nums1), sum(nums2)) = max(210, 80) = 210 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [20,40,20,70,30], nums2 = [50,20,50,40,20]
<strong>输出:</strong> 220
<strong>解释:</strong> 选择 left = 3 和 right = 4 ，得到 nums1 = [20,40,20,40,20] 和 nums2 = [50,20,50,70,30] 。
分数为 max(sum(nums1), sum(nums2)) = max(140, 220) = 220 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums1 = [7,11,13], nums2 = [1,1,1]
<strong>输出:</strong> 31
<strong>解释:</strong> 选择不交换任何子数组。
分数为 max(sum(nums1), sum(nums2)) = max(31, 3) = 31 。
</pre>

#### 提示:
* `n == nums1.length == nums2.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= nums1[i], nums2[i] <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximums_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let mut prefix_sum1 = vec![0; n + 1];
        let mut prefix_sum2 = vec![0; n + 1];
        let mut max_diff12 = 0;
        let mut max_diff21 = 0;
        let mut ret = 0;

        for i in 0..n {
            prefix_sum1[i + 1] = prefix_sum1[i] + nums1[i];
            prefix_sum2[i + 1] = prefix_sum2[i] + nums2[i];
        }

        for i in 0..n {
            max_diff12 = max_diff12.max(prefix_sum1[i] - prefix_sum2[i]);
            max_diff21 = max_diff21.max(prefix_sum2[i] - prefix_sum1[i]);
            ret = ret.max(prefix_sum1[n] + prefix_sum2[i + 1] - prefix_sum1[i + 1] + max_diff12);
            ret = ret.max(prefix_sum2[n] + prefix_sum1[i + 1] - prefix_sum2[i + 1] + max_diff21);
        }

        ret
    }
}
```
