# 1855. Maximum Distance Between a Pair of Values
You are given two **non-increasing 0-indexed** integer arrays `nums1` and `nums2`.

A pair of indices `(i, j)`, where `0 <= i < nums1.length` and `0 <= j < nums2.length`, is **valid** if both `i <= j` and `nums1[i] <= nums2[j]`. The **distance** of the pair is `j - i`.

Return *the **maximum distance** of any **valid** pair* `(i, j)`. *If there are no valid pairs, return* `0`.

An array `arr` is **non-increasing** if `arr[i-1] >= arr[i]` for every `1 <= i < arr.length`.

#### Example 1:
<pre>
<strong>Input:</strong> nums1 = [55,30,5,4,2], nums2 = [100,20,10,10,5]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The valid pairs are (0,0), (2,2), (2,3), (2,4), (3,3), (3,4), and (4,4).
The maximum distance is 2 with pair (2,4).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums1 = [2,2,2], nums2 = [10,10,1]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The valid pairs are (0,0), (0,1), and (1,1).
The maximum distance is 1 with pair (0,1).
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums1 = [30,29,19,5], nums2 = [25,25,25,25,25]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The valid pairs are (2,2), (2,3), (2,4), (3,3), and (3,4).
The maximum distance is 2 with pair (2,4).
</pre>

#### Constraints:
* <code>1 <= nums1.length, nums2.length <= 10<sup>5</sup></code>
* <code>1 <= nums1[i], nums2[j] <= 10<sup>5</sup></code>
* Both `nums1` and `nums2` are **non-increasing**.

## Solutions (Python)

### 1. Solution
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
