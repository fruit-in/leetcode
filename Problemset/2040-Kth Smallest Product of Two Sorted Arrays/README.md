# 2040. Kth Smallest Product of Two Sorted Arrays
Given two **sorted 0-indexed** integer arrays `nums1` and `nums2` as well as an integer `k`, return *the* <code>k<sup>th</sup></code> *(**1-based**) smallest product of* `nums1[i] * nums2[j]` *where* `0 <= i < nums1.length` *and* `0 <= j < nums2.length`.

#### Example 1:
<pre>
<strong>Input:</strong> nums1 = [2,5], nums2 = [3,4], k = 2
<strong>Output:</strong> 8
<strong>Explanation:</strong> The 2 smallest products are:
- nums1[0] * nums2[0] = 2 * 3 = 6
- nums1[0] * nums2[1] = 2 * 4 = 8
The 2nd smallest product is 8.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums1 = [-4,-2,0,3], nums2 = [2,4], k = 6
<strong>Output:</strong> 0
<strong>Explanation:</strong> The 6 smallest products are:
- nums1[0] * nums2[1] = (-4) * 4 = -16
- nums1[0] * nums2[0] = (-4) * 2 = -8
- nums1[1] * nums2[1] = (-2) * 4 = -8
- nums1[1] * nums2[0] = (-2) * 2 = -4
- nums1[2] * nums2[0] = 0 * 2 = 0
- nums1[2] * nums2[1] = 0 * 4 = 0
The 6th smallest product is 0.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums1 = [-2,-1,0,1,2], nums2 = [-3,-1,2,4,5], k = 3
<strong>Output:</strong> -6
<strong>Explanation:</strong> The 3 smallest products are:
- nums1[0] * nums2[4] = (-2) * 5 = -10
- nums1[0] * nums2[3] = (-2) * 4 = -8
- nums1[4] * nums2[0] = 2 * (-3) = -6
The 3rd smallest product is -6.
</pre>

#### Constraints:
* <code>1 <= nums1.length, nums2.length <= 5 * 10<sup>4</sup></code>
* <code>-10<sup>5</sup> <= nums1[i], nums2[j] <= 10<sup>5</sup></code>
* `1 <= k <= nums1.length * nums2.length`
* `nums1` and `nums2` are sorted.

## Solutions (Python)

### 1. Solution
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
