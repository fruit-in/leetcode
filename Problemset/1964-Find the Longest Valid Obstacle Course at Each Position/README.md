# 1964. Find the Longest Valid Obstacle Course at Each Position
You want to build some obstacle courses. You are given a **0-indexed** integer array `obstacles` of length `n`, where `obstacles[i]` describes the height of the <code>i<sup>th</sup></code> obstacle.

For every index `i` between `0` and `n - 1` (**inclusive**), find the length of the **longest obstacle course** in `obstacles` such that:

* You choose any number of obstacles between `0` and `i` **inclusive**.
* You must include the <code>i<sup>th</sup></code> obstacle in the course.
* You must put the chosen obstacles in the **same order** as they appear in `obstacles`.
* Every obstacle (except the first) is **taller** than or the **same height** as the obstacle immediately before it.

Return *an array* `ans` *of length* `n`, *where* `ans[i]` *is the length of the **longest obstacle course** for index* `i` *as described above*.

#### Example 1:
<pre>
<strong>Input:</strong> obstacles = [1,2,3,2]
<strong>Output:</strong> [1,2,3,3]
<strong>Explanation:</strong> The longest valid obstacle course at each position is:
- i = 0: [1], [1] has length 1.
- i = 1: [1,2], [1,2] has length 2.
- i = 2: [1,2,3], [1,2,3] has length 3.
- i = 3: [1,2,3,2], [1,2,2] has length 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> obstacles = [2,2,1]
<strong>Output:</strong> [1,2,1]
<strong>Explanation:</strong> The longest valid obstacle course at each position is:
- i = 0: [2], [2] has length 1.
- i = 1: [2,2], [2,2] has length 2.
- i = 2: [2,2,1], [1] has length 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> obstacles = [3,1,5,6,4,2]
<strong>Output:</strong> [1,1,2,3,2,2]
<strong>Explanation:</strong> The longest valid obstacle course at each position is:
- i = 0: [3], [3] has length 1.
- i = 1: [3,1], [1] has length 1.
- i = 2: [3,1,5], [3,5] has length 2. [1,5] is also valid.
- i = 3: [3,1,5,6], [3,5,6] has length 3. [1,5,6] is also valid.
- i = 4: [3,1,5,6,4], [3,4] has length 2. [1,4] is also valid.
- i = 5: [3,1,5,6,4,2], [1,2] has length 2.
</pre>

#### Constraints:
* `n == obstacles.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= obstacles[i] <= 10<sup>7</sup></code>

## Solutions (Python)

### 1. Solution
```Python
from sortedcontainers import SortedList


class Solution:
    def longestObstacleCourseAtEachPosition(self, obstacles: List[int]) -> List[int]:
        n = len(obstacles)
        sl = SortedList([(0, 0)])
        ans = [1] * n

        for i in range(n):
            j = sl.bisect_left((obstacles[i], n)) - 1
            ans[i] = sl[j][1] + 1

            if j + 1 < len(sl) and sl[j + 1][1] == ans[i]:
                sl.pop(j + 1)
            if sl[j][0] == obstacles[i]:
                sl.pop(j)

            sl.add((obstacles[i], ans[i]))

        return ans
```
