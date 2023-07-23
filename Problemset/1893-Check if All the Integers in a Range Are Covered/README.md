# 1893. Check if All the Integers in a Range Are Covered
You are given a 2D integer array `ranges` and two integers `left` and `right`. Each `ranges[i] = [starti, endi]` represents an **inclusive** interval between `starti` and `endi`.

Return `true` *if each integer in the inclusive range* `[left, right]` *is covered by **at least one** interval in* `ranges`. Return `false` *otherwise*.

An integer `x` is covered by an interval `ranges[i] = [starti, endi]` if `starti <= x <= endi`.

#### Example 1:
<pre>
<strong>Input:</strong> ranges = [[1,2],[3,4],[5,6]], left = 2, right = 5
<strong>Output:</strong> true
<strong>Explanation:</strong> Every integer between 2 and 5 is covered:
- 2 is covered by the first range.
- 3 and 4 are covered by the second range.
- 5 is covered by the third range.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> ranges = [[1,10],[10,20]], left = 21, right = 21
<strong>Output:</strong> false
<strong>Explanation:</strong> 21 is not covered by any range.
</pre>

#### Constraints:
* `1 <= ranges.length <= 50`
* `1 <= starti <= endi <= 50`
* `1 <= left <= right <= 50`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def isCovered(self, ranges: List[List[int]], left: int, right: int) -> bool:
        for start, end in sorted(ranges):
            if start > left or left > right:
                break
            elif end >= left:
                left = end + 1

        return left > right
```
