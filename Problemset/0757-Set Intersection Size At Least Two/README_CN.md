# 757. 设置交集大小至少为2
给你一个二维整数数组 `intervals` ，其中 <code>intervals[i] = [start<sub>i</sub>, end<sub>i</sub>]</code> 表示从 <code>start<sub>i</sub></code> 到 <code>end<sub>i</sub></code> 的所有整数，包括 <code>start<sub>i</sub></code> 和 <code>end<sub>i</sub></code> 。

**包含集合** 是一个名为 `nums` 的数组，并满足 `intervals` 中的每个区间都 **至少** 有 **两个** 整数在 `nums` 中。

* 例如，如果 `intervals = [[1,3], [3,7], [8,9]]` ，那么 `[1,2,4,7,8,9]` 和 `[2,3,4,8,9]` 都符合 **包含集合** 的定义。

返回包含集合可能的最小大小。

#### 示例 1:
<pre>
<strong>输入:</strong> intervals = [[1,3],[3,7],[8,9]]
<strong>输出:</strong> 5
<strong>解释:</strong> nums = [2, 3, 4, 8, 9].
可以证明不存在元素数量为 4 的包含集合。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> intervals = [[1,3],[1,4],[2,5],[3,5]]
<strong>输出:</strong> 3
<strong>解释:</strong> nums = [2, 3, 4].
可以证明不存在元素数量为 2 的包含集合。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> intervals = [[1,2],[2,3],[2,4],[4,5]]
<strong>输出:</strong> 5
<strong>解释:</strong> nums = [1, 2, 3, 4, 5].
可以证明不存在元素数量为 4 的包含集合。
</pre>

#### 提示:
* `1 <= intervals.length <= 3000`
* `intervals[i].length == 2`
* <code>0 <= starti < endi <= 10<sup>8</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def intersectionSizeTwo(self, intervals: List[List[int]]) -> int:
        intervals.sort(key=lambda interval: (interval[1], interval[0]))
        nums = [intervals[0][1] - 1, intervals[0][1]]

        for start, end in intervals[1:]:
            if start <= nums[-2]:
                continue
            elif start > nums[-1]:
                nums.append(end - 1)
                nums.append(end)
            elif end > nums[-1]:
                nums.append(end)
            else:
                nums.pop()
                nums.append(end - 1)
                nums.append(end)

        return len(nums)
```
