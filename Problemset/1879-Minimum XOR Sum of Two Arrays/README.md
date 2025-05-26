# 1879. Minimum XOR Sum of Two Arrays
You are given two integer arrays `nums1` and `nums2` of length `n`.

The **XOR sum** of the two integer arrays is `(nums1[0] XOR nums2[0]) + (nums1[1] XOR nums2[1]) + ... + (nums1[n - 1] XOR nums2[n - 1])` (**0-indexed**).

* For example, the **XOR sum** of `[1,2,3]` and `[3,2,1]` is equal to `(1 XOR 3) + (2 XOR 2) + (3 XOR 1) = 2 + 0 + 2 = 4`.

Rearrange the elements of `nums2` such that the resulting **XOR sum** is **minimized**.

Return *the **XOR sum** after the rearrangement*.

#### Example 1:
<pre>
<strong>Input:</strong> nums1 = [1,2], nums2 = [2,3]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Rearrange nums2 so that it becomes [3,2].
The XOR sum is (1 XOR 3) + (2 XOR 2) = 2 + 0 = 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums1 = [1,0,3], nums2 = [5,3,4]
<strong>Output:</strong> 8
<strong>Explanation:</strong> Rearrange nums2 so that it becomes [5,4,3].
The XOR sum is (1 XOR 5) + (0 XOR 4) + (3 XOR 3) = 4 + 4 + 0 = 8.
</pre>

#### Constraints:
* `n == nums1.length`
* `n == nums2.length`
* `1 <= n <= 14`
* <code>0 <= nums1[i], nums2[i] <= 10<sup>7</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def minimumXORSum(self, nums1: List[int], nums2: List[int]) -> int:
        @cache
        def dfs(i: int, mask: int) -> int:
            if i == len(nums1):
                return 0

            ret = inf

            for j in range(len(nums2)):
                if (mask >> j) & 1 == 0:
                    ret = min(ret, (nums1[i] ^ nums2[j]) +
                              dfs(i + 1, mask | (1 << j)))

            return ret

        return dfs(0, 0)
```
