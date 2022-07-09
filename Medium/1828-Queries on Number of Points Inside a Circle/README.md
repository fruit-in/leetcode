# 1828. Queries on Number of Points Inside a Circle
You are given an array `points` where `points[i] = [xi, yi]` is the coordinates of the `ith` point on a 2D plane. Multiple points can have the **same** coordinates.

You are also given an array `queries` where `queries[j] = [xj, yj, rj]` describes a circle centered at `(xj, yj)` with a radius of `rj`.

For each query `queries[j]`, compute the number of points **inside** the `jth` circle. Points **on the border** of the circle are considered **inside**.

Return *an array* `answer`, *where* `answer[j]` *is the answer to the* `jth` *query*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/03/25/chrome_2021-03-25_22-34-16.png)
<pre>
<strong>Input:</strong> points = [[1,3],[3,3],[5,3],[2,2]], queries = [[2,3,1],[4,3,1],[1,1,2]]
<strong>Output:</strong> [3,2,2]
<strong>Explanation:</strong> The points and circles are shown above.
queries[0] is the green circle, queries[1] is the red circle, and queries[2] is the blue circle.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/03/25/chrome_2021-03-25_22-42-07.png)
<pre>
<strong>Input:</strong> points = [[1,1],[2,2],[3,3],[4,4],[5,5]], queries = [[1,2,2],[2,2,2],[4,3,2],[4,3,3]]
<strong>Output:</strong> [2,3,2,4]
<strong>Explanation:</strong> The points and circles are shown above.
queries[0] is green, queries[1] is red, queries[2] is blue, and queries[3] is purple.
</pre>

#### Constraints:
* `1 <= points.length <= 500`
* `points[i].length == 2`
* `0 <= xi, yi <= 500`
* `1 <= queries.length <= 500`
* `queries[j].length == 3`
* `0 <= xj, yj <= 500`
* `1 <= rj <= 500`
* All coordinates are integers.

**Follow up:** Could you find the answer for each query in better complexity than `O(n)`?


## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def countPoints(self, points: List[List[int]], queries: List[List[int]]) -> List[int]:
        answer = [0] * len(queries)

        for i in range(len(points)):
            xi, yi = points[i]
            for j in range(len(queries)):
                xj, yj, rj = queries[j]
                if (xi - xj) * (xi - xj) + (yi - yj) * (yi - yj) <= rj * rj:
                    answer[j] += 1

        return answer
```
