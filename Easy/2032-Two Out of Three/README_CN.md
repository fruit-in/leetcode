# 2032. 至少在两个数组中出现的值
给你三个整数数组 `nums1`、`nums2` 和 `nums3` ，请你构造并返回一个 **元素各不相同的** 数组，且由 **至少** 在 **两个** 数组中出现的所有值组成。数组中的元素可以按 **任意** 顺序排列。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [1,1,3,2], nums2 = [2,3], nums3 = [3]
<strong>输出:</strong> [3,2]
<strong>解释:</strong> 至少在两个数组中出现的所有值为：
- 3 ，在全部三个数组中都出现过。
- 2 ，在数组 nums1 和 nums2 中出现过。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [3,1], nums2 = [2,3], nums3 = [1,2]
<strong>输出:</strong> [2,3,1]
<strong>解释:</strong> 至少在两个数组中出现的所有值为：
- 2 ，在数组 nums2 和 nums3 中出现过。
- 3 ，在数组 nums1 和 nums2 中出现过。
- 1 ，在数组 nums1 和 nums3 中出现过。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums1 = [1,2,2], nums2 = [4,3,3], nums3 = [5]
<strong>输出:</strong> []
<strong>解释:</strong> 不存在至少在两个数组中出现的值。
</pre>

#### 提示:
* `1 <= nums1.length, nums2.length, nums3.length <= 100`
* `1 <= nums1[i], nums2[j], nums3[k] <= 100`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def twoOutOfThree(self, nums1: List[int], nums2: List[int], nums3: List[int]) -> List[int]:
        nums1 = set(nums1)
        nums2 = set(nums2)
        nums3 = set(nums3)
        nums12 = nums1.intersection(nums2)
        nums13 = nums1.intersection(nums3)
        nums23 = nums2.intersection(nums3)

        return list(nums12.union(nums13).union(nums23))
```
