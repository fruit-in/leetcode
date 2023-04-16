# 1855. 下标对中的最大距离
给你两个 **非递增** 的整数数组 `nums1` 和 `nums2` ，数组下标均 **从 0 开始** 计数。

下标对 `(i, j)` 中 `0 <= i < nums1.length` 且 `0 <= j < nums2.length` 。如果该下标对同时满足 `i <= j` 且 `nums1[i] <= nums2[j]` ，则称之为 **有效** 下标对，该下标对的 **距离** 为 `j - i` 。

返回所有 **有效** 下标对 `(i, j)` 中的 **最大距离** 。如果不存在有效下标对，返回 `0` 。

一个数组 `arr` ，如果每个 `1 <= i < arr.length` 均有 `arr[i-1] >= arr[i]` 成立，那么该数组是一个 **非递增** 数组。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [55,30,5,4,2], nums2 = [100,20,10,10,5]
<strong>输出:</strong> 2
<strong>解释:</strong> 有效下标对是 (0,0), (2,2), (2,3), (2,4), (3,3), (3,4) 和 (4,4) 。
最大距离是 2 ，对应下标对 (2,4) 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [2,2,2], nums2 = [10,10,1]
<strong>输出:</strong> 1
<strong>解释:</strong> 有效下标对是 (0,0), (0,1) 和 (1,1) 。
最大距离是 1 ，对应下标对 (0,1) 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums1 = [30,29,19,5], nums2 = [25,25,25,25,25]
<strong>输出:</strong> 2
<strong>解释:</strong> 有效下标对是 (2,2), (2,3), (2,4), (3,3) 和 (3,4) 。
最大距离是 2 ，对应下标对 (2,4) 。
</pre>

#### 提示:
* <code>1 <= nums1.length <= 10<sup>5</sup></code>
* <code>1 <= nums2.length <= 10<sup>5</sup></code>
* <code>1 <= nums1[i], nums2[j] <= 10<sup>5</sup></code>
* `nums1` 和 `nums2` 都是 **非递增** 数组

## 题解 (Python)

### 1. 题解
```Python
import bisect


class Solution:
    def maxDistance(self, nums1: List[int], nums2: List[int]) -> int:
        ret = 0

        for i in range(len(nums1)):
            j = bisect.bisect_right(
                nums2, False, key=lambda x: x < nums1[i]) - 1
            ret = max(ret, j - i)

        return ret
```
