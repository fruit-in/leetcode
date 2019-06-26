# 1051. Height Checker
Students are asked to stand in non-decreasing order of heights for an annual photo.

Return the minimum number of students not standing in the right positions.  (This is the number of students that must move in order for all students to be standing in non-decreasing order of height.)

#### Example 1:
<pre>
<strong>Input:</strong> [1,1,4,2,1,3]
<strong>Output:</strong> 3
<strong>Explanation:</strong> 
Students with heights 4, 3 and the last 1 are not standing in the right positions.
</pre>

#### Note:
1. <code>1 <= heights.length <= 100</code>
2. <code>1 <= heights[i] <= 100</code>

## Solutions

### 1. Solution (Python3)
```Python3
class Solution:
    def heightChecker(self, heights: List[int]) -> int:
        return len(list(filter(lambda x : x[0] != x[1], zip(sorted(heights), heights))))
```
