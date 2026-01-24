# 947. Most Stones Removed with Same Row or Column
On a 2D plane, we place `n` stones at some integer coordinate points. Each coordinate point may have at most one stone.

A stone can be removed if it shares either **the same row or the same column** as another stone that has not been removed.

Given an array `stones` of length `n` where <code>stones[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> represents the location of the <code>i<sup>th</sup></code> stone, return *the largest possible number of stones that can be removed*.

#### Example 1:
<pre>
<strong>Input:</strong> stones = [[0,0],[0,1],[1,0],[1,2],[2,1],[2,2]]
<strong>Output:</strong> 5
<strong>Explanation:</strong> One way to remove 5 stones is as follows:
1. Remove stone [2,2] because it shares the same row as [2,1].
2. Remove stone [2,1] because it shares the same column as [0,1].
3. Remove stone [1,2] because it shares the same row as [1,0].
4. Remove stone [1,0] because it shares the same column as [0,0].
5. Remove stone [0,1] because it shares the same row as [0,0].
Stone [0,0] cannot be removed since it does not share a row/column with another stone still on the plane.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> stones = [[0,0],[0,2],[1,1],[2,0],[2,2]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> One way to make 3 moves is as follows:
1. Remove stone [2,2] because it shares the same row as [2,0].
2. Remove stone [2,0] because it shares the same column as [0,0].
3. Remove stone [0,2] because it shares the same row as [0,0].
Stones [0,0] and [1,1] cannot be removed since they do not share a row/column with another stone still on the plane.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> stones = [[0,0]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> [0,0] is the only stone on the plane, so you cannot remove it.
</pre>

#### Constraints:
* `1 <= stones.length <= 1000`
* <code>0 <= x<sub>i</sub>, y<sub>i</sub> <= 10<sup>4</sup></code>
* No two stones are at the same coordinate point.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def removeStones(self, stones: List[List[int]]) -> int:
        n = len(stones)
        parent = {(x, y): (x, y) for x, y in stones}
        group = set()
        stones.sort()

        for i in range(1, n):
            xi, yi = stones[i]
            xj, yj = stones[i - 1]

            if xi != xj:
                continue

            while parent[(xi, yi)] != parent[parent[(xi, yi)]]:
                parent[(xi, yi)] = parent[parent[(xi, yi)]]
            while parent[(xj, yj)] != parent[parent[(xj, yj)]]:
                parent[(xj, yj)] = parent[parent[(xj, yj)]]

            parent[parent[(xi, yi)]] = parent[(xj, yj)]

        stones.sort(key=lambda s: s[1])

        for i in range(1, n):
            xi, yi = stones[i]
            xj, yj = stones[i - 1]

            if yi != yj:
                continue

            while parent[(xi, yi)] != parent[parent[(xi, yi)]]:
                parent[(xi, yi)] = parent[parent[(xi, yi)]]
            while parent[(xj, yj)] != parent[parent[(xj, yj)]]:
                parent[(xj, yj)] = parent[parent[(xj, yj)]]

            parent[parent[(xi, yi)]] = parent[(xj, yj)]

        for x, y in stones:
            while parent[(x, y)] != parent[parent[(x, y)]]:
                parent[(x, y)] = parent[parent[(x, y)]]

            group.add(parent[(x, y)])

        return n - len(group)
```
