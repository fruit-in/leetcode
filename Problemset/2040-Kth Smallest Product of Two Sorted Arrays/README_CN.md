# 2040. 两个有序数组的第 K 小乘积
给你两个 **从小到大排好序** 且下标从 **0** 开始的整数数组 `nums1` 和 `nums2` 以及一个整数 `k` ，请你返回第 `k` （从 **1** 开始编号）小的 `nums1[i] * nums2[j]` 的乘积，其中 `0 <= i < nums1.length` 且 `0 <= j < nums2.length` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [2,5], nums2 = [3,4], k = 2
<strong>输出:</strong> 8
<strong>解释:</strong> 第 2 小的乘积计算如下：
- nums1[0] * nums2[0] = 2 * 3 = 6
- nums1[0] * nums2[1] = 2 * 4 = 8
第 2 小的乘积为 8 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [-4,-2,0,3], nums2 = [2,4], k = 6
<strong>输出:</strong> 0
<strong>解释:</strong> 第 6 小的乘积计算如下：
- nums1[0] * nums2[1] = (-4) * 4 = -16
- nums1[0] * nums2[0] = (-4) * 2 = -8
- nums1[1] * nums2[1] = (-2) * 4 = -8
- nums1[1] * nums2[0] = (-2) * 2 = -4
- nums1[2] * nums2[0] = 0 * 2 = 0
- nums1[2] * nums2[1] = 0 * 4 = 0
第 6 小的乘积为 0 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums1 = [-2,-1,0,1,2], nums2 = [-3,-1,2,4,5], k = 3
<strong>输出:</strong> -6
<strong>解释:</strong> 第 3 小的乘积计算如下：
- nums1[0] * nums2[4] = (-2) * 5 = -10
- nums1[0] * nums2[3] = (-2) * 4 = -8
- nums1[4] * nums2[0] = 2 * (-3) = -6
第 3 小的乘积为 -6 。
</pre>

#### 提示:
* <code>1 <= nums1.length, nums2.length <= 5 * 10<sup>4</sup></code>
* <code>-10<sup>5</sup> <= nums1[i], nums2[j] <= 10<sup>5</sup></code>
* `1 <= k <= nums1.length * nums2.length`
* `nums1` 和 `nums2` 都是从小到大排好序的。

## 题解 (Python)

### 1. 题解
```Python
import math


class Solution:
    def kthSmallestProduct(self, nums1: List[int], nums2: List[int], k: int) -> int:
        if len(nums2) < len(nums1):
            nums1, nums2 = nums2, nums1

        lo = min(nums1[0] * nums2[0], nums1[0] * nums2[-1],
                 nums1[-1] * nums2[0], nums1[-1] * nums2[-1])
        hi = max(nums1[0] * nums2[0], nums1[0] * nums2[-1],
                 nums1[-1] * nums2[0], nums1[-1] * nums2[-1])

        while lo < hi:
            mid = (lo + hi) // 2
            count = 0

            for i in range(len(nums1)):
                if nums1[i] == 0:
                    count += len(nums2) if mid >= 0 else 0
                elif nums1[i] > 0:
                    count += bisect.bisect(nums2, mid // nums1[i])
                else:
                    count += len(nums2) - bisect.bisect(nums2,
                                                        math.ceil(mid / nums1[i]) - 1)

            if count < k:
                lo = mid + 1
            else:
                hi = mid

        return hi
```
