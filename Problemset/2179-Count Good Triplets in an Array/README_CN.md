# 2179. 统计数组中好三元组数目
给你两个下标从 **0** 开始且长度为 `n` 的整数数组 `nums1` 和 `nums2` ，两者都是 `[0, 1, ..., n - 1]` 的 **排列** 。

**好三元组** 指的是 `3` 个 **互不相同** 的值，且它们在数组 `nums1` 和 `nums2` 中出现顺序保持一致。换句话说，如果我们将 <code>pos1<sub>v</sub></code> 记为值 `v` 在 `nums1` 中出现的位置，<code>pos2<sub>v</sub></code> 为值 `v` 在 `nums2` 中的位置，那么一个好三元组定义为 `0 <= x, y, z <= n - 1` ，且 <code>pos1<sub>x</sub> < pos1<sub>y</sub> < pos1<sub>z</sub></code> 和 <code>pos2<sub>x</sub> < pos2<sub>y</sub> < pos2<sub>z</sub></code> 都成立的 (x, y, z) 。

请你返回好三元组的 **总数目** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [2,0,1,3], nums2 = [0,1,2,3]
<strong>输出:</strong> 1
<strong>解释:</strong>
总共有 4 个三元组 (x,y,z) 满足 pos1x < pos1y < pos1z ，分别是 (2,0,1) ，(2,0,3) ，(2,1,3) 和 (0,1,3) 。
这些三元组中，只有 (0,1,3) 满足 pos2x < pos2y < pos2z 。所以只有 1 个好三元组。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [4,0,1,3,2], nums2 = [4,1,0,2,3]
<strong>输出:</strong> 4
<strong>解释:</strong> 总共有 4 个好三元组 (4,0,3) ，(4,0,2) ，(4,1,3) 和 (4,1,2) 。
</pre>

#### 提示:
* `n == nums1.length == nums2.length`
* <code>3 <= n <= 10<sup>5</sup></code>
* `0 <= nums1[i], nums2[i] <= n - 1`
* `nums1` 和 `nums2` 是 `[0, 1, ..., n - 1]` 的排列。

## 题解 (Python)

### 1. 题解
```Python
from sortedcontainers import SortedList


class Solution:
    def goodTriplets(self, nums1: List[int], nums2: List[int]) -> int:
        pos1 = {x: i for i, x in enumerate(nums1)}
        pos1x = SortedList()
        pos1z = SortedList()
        count = [0] * len(nums2)
        ret = 0

        for i in range(len(nums2)):
            count[i] = pos1x.bisect_left(pos1[nums2[i]])
            pos1x.add(pos1[nums2[i]])

        for i in range(len(nums2) - 1, -1, -1):
            ret += count[i] * (len(pos1z) - pos1z.bisect_left(pos1[nums2[i]]))
            pos1z.add(pos1[nums2[i]])

        return ret
```
