# 757. Set Intersection Size At Least Two
You are given a 2D integer array `intervals` where <code>intervals[i] = [start<sub>i</sub>, end<sub>i</sub>]</code> represents all the integers from <code>start<sub>i</sub></code> to <code>end<sub>i</sub></code> inclusively.

A **containing set** is an array `nums` where each interval from `intervals` has **at least two** integers in `nums`.

* For example, if `intervals = [[1,3], [3,7], [8,9]]`, then `[1,2,4,7,8,9]` and `[2,3,4,8,9]` are **containing sets**.

Return *the minimum possible size of a containing set*.

#### Example 1:
<pre>
<strong>Input:</strong> intervals = [[1,3],[3,7],[8,9]]
<strong>Output:</strong> 5
<strong>Explanation:</strong> let nums = [2, 3, 4, 8, 9].
It can be shown that there cannot be any containing array of size 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> intervals = [[1,3],[1,4],[2,5],[3,5]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> let nums = [2, 3, 4].
It can be shown that there cannot be any containing array of size 2.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> intervals = [[1,2],[2,3],[2,4],[4,5]]
<strong>Output:</strong> 5
<strong>Explanation:</strong> let nums = [1, 2, 3, 4, 5].
It can be shown that there cannot be any containing array of size 4.
</pre>

#### Constraints:
* `1 <= intervals.length <= 3000`
* `intervals[i].length == 2`
* <code>0 <= starti < endi <= 10<sup>8</sup></code>

## Solutions (Python)

### 1. Solution
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
