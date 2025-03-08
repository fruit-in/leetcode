# 1039. Minimum Score Triangulation of Polygon
You have a convex `n`-sided polygon where each vertex has an integer value. You are given an integer array `values` where `values[i]` is the value of the <code>i<sup>th</sup></code> vertex in **clockwise order**.

**Polygon triangulation** is a process where you divide a polygon into a set of triangles and the vertices of each triangle must also be vertices of the original polygon. Note that no other shapes other than triangles are allowed in the division. This process will result in `n - 2` triangles.

You will **triangulate** the polygon. For each triangle, the *weight* of that triangle is the product of the values at its vertices. The total score of the triangulation is the sum of these *weights* over all `n - 2` triangles.

Return the *minimum possible score* that you can achieve with some **triangulation** of the polygon.

#### Example 1:
<pre>
<strong>Input:</strong> values = [1,2,3]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The polygon is already triangulated, and the score of the only triangle is 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> values = [3,7,4,5]
<strong>Output:</strong> 144
<strong>Explanation:</strong> There are two triangulations, with possible scores: 3*7*5 + 4*5*7 = 245, or 3*4*5 + 3*4*7 = 144.
The minimum score is 144.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> values = [1,3,1,4,1,5]
<strong>Output:</strong> 13
<strong>Explanation:</strong> The minimum score triangulation is 1*1*3 + 1*1*4 + 1*1*5 + 1*1*1 = 13.
</pre>

#### Constraints:
* `n == values.length`
* `3 <= n <= 50`
* `1 <= values[i] <= 100`

## Solutions (Python)

### 1. Solution
```Python
from functools import cache


class Solution:
    def minScoreTriangulation(self, values: List[int]) -> int:
        @cache
        def subMinScore(i: int, j: int) -> int:
            return min((values[i] * values[j] * values[k] + subMinScore(i, k) + subMinScore(k, j) for k in range(i + 1, j)), default=0)

        return subMinScore(0, len(values) - 1)
```
