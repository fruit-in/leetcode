# 1879. 两个数组最小的异或值之和
给你两个整数数组 `nums1` 和 `nums2` ，它们长度都为 `n` 。

两个数组的 **异或值之和** 为 `(nums1[0] XOR nums2[0]) + (nums1[1] XOR nums2[1]) + ... + (nums1[n - 1] XOR nums2[n - 1])` （**下标从 0 开始**）。

* 比方说，`[1,2,3]` 和 `[3,2,1]` 的 **异或值之和** 等于 `(1 XOR 3) + (2 XOR 2) + (3 XOR 1) = 2 + 0 + 2 = 4` 。

请你将 `nums2` 中的元素重新排列，使得 **异或值之和 最小** 。

请你返回重新排列之后的 **异或值之和** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [1,2], nums2 = [2,3]
<strong>输出:</strong> 2
<strong>解释:</strong> 将 nums2 重新排列得到 [3,2] 。
异或值之和为 (1 XOR 3) + (2 XOR 2) = 2 + 0 = 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [1,0,3], nums2 = [5,3,4]
<strong>输出:</strong> 8
<strong>解释:</strong> 将 nums2 重新排列得到 [5,4,3] 。
异或值之和为 (1 XOR 5) + (0 XOR 4) + (3 XOR 3) = 4 + 4 + 0 = 8 。
</pre>

#### 提示:
* `n == nums1.length`
* `n == nums2.length`
* `1 <= n <= 14`
* <code>0 <= nums1[i], nums2[i] <= 10<sup>7</sup></code>

## 题解 (Python)

### 1. 题解
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
