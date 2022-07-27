# 1577. 数的平方等于两数乘积的方法数
给你两个整数数组 `nums1` 和 `nums2` ，请你返回根据以下规则形成的三元组的数目（类型 1 和类型 2 ）：
* 类型 1：三元组 `(i, j, k)` ，如果 <code>nums1[i]<sup>2</sup> == nums2[j] * nums2[k]</code> 其中 `0 <= i < nums1.length` 且 `0 <= j < k < nums2.length`
* 类型 2：三元组 `(i, j, k)` ，如果 <code>nums2[i]<sup>2</sup> == nums1[j] * nums1[k]</code> 其中 `0 <= i < nums2.length` 且 `0 <= j < k < nums1.length`

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [7,4], nums2 = [5,2,8,9]
<strong>输出:</strong> 1
<strong>解释:</strong> 类型 1：(1,1,2), nums1[1]^2 = nums2[1] * nums2[2] (4^2 = 2 * 8)
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [1,1], nums2 = [1,1,1]
<strong>输出:</strong> 9
<strong>解释:</strong> 所有三元组都符合题目要求，因为 1^2 = 1 * 1
类型 1：(0,0,1), (0,0,2), (0,1,2), (1,0,1), (1,0,2), (1,1,2), nums1[i]^2 = nums2[j] * nums2[k]
类型 2：(0,0,1), (1,0,1), (2,0,1), nums2[i]^2 = nums1[j] * nums1[k]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums1 = [7,7,8,3], nums2 = [1,2,9,7]
<strong>输出:</strong> 2
<strong>解释:</strong> 有两个符合题目要求的三元组
类型 1：(3,0,2), nums1[3]^2 = nums2[0] * nums2[2]
类型 2：(3,0,1), nums2[3]^2 = nums1[0] * nums1[1]
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> nums1 = [4,7,9,11,23], nums2 = [3,5,1024,12,18]
<strong>输出:</strong> 0
<strong>解释:</strong> 不存在符合题目要求的三元组
</pre>

#### 提示:
* `1 <= nums1.length, nums2.length <= 1000`
* <code>1 <= nums1[i], nums2[i] <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def numTriplets(self, nums1: List[int], nums2: List[int]) -> int:
        count1 = Counter(nums1)
        count2 = Counter(nums2)
        ret = 0

        for x in nums1:
            x2 = x * x
            for y in nums2:
                if x2 % y == 0 and x2 // y in count2:
                    ret += count2[x2 // y]
                    if y == x:
                        ret -= 1
        for x in nums2:
            x2 = x * x
            for y in nums1:
                if x2 % y == 0 and x2 // y in count1:
                    ret += count1[x2 // y]
                    if y == x:
                        ret -= 1

        return ret // 2
```
