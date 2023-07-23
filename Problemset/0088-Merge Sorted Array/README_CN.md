# 88. 合并两个有序数组
给定两个有序整数数组 *nums1* 和 *nums2*，将 *nums2* 合并到 *nums1* 中，使得 *num1* 成为一个有序数组。

#### 说明:
* 初始化 *nums1* 和 *nums2* 的元素数量分别为 *m* 和 *n*。
* 你可以假设 *nums1* 有足够的空间（空间大小大于或等于 *m* + *n*）来保存 *nums2* 中的元素。

#### 示例:
<pre>
<strong>输入:</strong>
nums1 = [1,2,3,0,0,0], m = 3
nums2 = [2,5,6],       n = 3

<strong>输出:</strong> [1,2,2,3,5,6]
</pre>

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        """
        Do not return anything, modify nums1 in-place instead.
        """
        while m > 0 or n > 0:
            if n == 0 or (m > 0 and nums1[m - 1] >= nums2[n - 1]):
                nums1[m + n - 1] = nums1[m - 1]
                m -= 1
            else:
                nums1[m + n - 1] = nums2[n - 1]
                n -= 1
```

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} nums1
# @param {Integer} m
# @param {Integer[]} nums2
# @param {Integer} n
# @return {Void} Do not return anything, modify nums1 in-place instead.
def merge(nums1, m, nums2, n)
    while m > 0 or n > 0
        if n == 0 or (m > 0 and nums1[m - 1] >= nums2[n - 1])
            nums1[m + n - 1] = nums1[m - 1]
            m -= 1
        else
            nums1[m + n - 1] = nums2[n - 1]
            n -= 1
        end
    end
end
```
