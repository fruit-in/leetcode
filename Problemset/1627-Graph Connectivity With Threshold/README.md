# 1627. Graph Connectivity With Threshold
We have `n` cities labeled from `1` to `n`. Two different cities with labels `x` and `y` are directly connected by a bidirectional road if and only if `x` and `y` share a common divisor **strictly greater** than some `threshold`. More formally, cities with labels `x` and `y` have a road between them if there exists an integer `z` such that all of the following are true:
* `x % z == 0`,
* `y % z == 0`, and
* `z > threshold`.

Given the two integers, `n` and `threshold`, and an array of `queries`, you must determine for each <code>queries[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> if cities <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code> are connected directly or indirectly. (i.e. there is some path between them).

Return *an array* `answer`*, where* `answer.length == queries.length` *and* `answer[i]` *is* `true` *if for the* <code>i<sup>th</sup></code> *query, there is a path between* <code>a<sub>i</sub></code> *and* <code>b<sub>i</sub></code>*, or* `answer[i]` *is* `false` *if there is no path*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/10/09/ex1.jpg)
<pre>
<strong>Input:</strong> n = 6, threshold = 2, queries = [[1,4],[2,5],[3,6]]
<strong>Output:</strong> [false,false,true]
<strong>Explanation:</strong> The divisors for each number:
1:   1
2:   1, 2
3:   1, 3
4:   1, 2, 4
5:   1, 5
6:   1, 2, 3, 6
Using the underlined divisors above the threshold, only cities 3 and 6 share a common divisor, so they are the
only ones directly connected. The result of each query:
[1,4]   1 is not connected to 4
[2,5]   2 is not connected to 5
[3,6]   3 is connected to 6 through path 3--6
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/10/10/tmp.jpg)
<pre>
<strong>Input:</strong> n = 6, threshold = 0, queries = [[4,5],[3,4],[3,2],[2,6],[1,3]]
<strong>Output:</strong> [true,true,true,true,true]
<strong>Explanation:</strong> The divisors for each number are the same as the previous example. However, since the threshold is 0,
all divisors can be used. Since all numbers share 1 as a divisor, all cities are connected.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2020/10/17/ex3.jpg)
<pre>
<strong>Input:</strong> n = 5, threshold = 1, queries = [[4,5],[4,5],[3,2],[2,3],[3,4]]
<strong>Output:</strong> [false,false,false,false,false]
<strong>Explanation:</strong> Only cities 2 and 4 share a common divisor 2 which is strictly greater than the threshold 1, so they are the only ones directly connected.
Please notice that there can be multiple queries for the same pair of nodes [x, y], and that the query [x, y] is equivalent to the query [y, x].
</pre>

#### Constraints:
* <code>2 <= n <= 10<sup>4</sup></code>
* `0 <= threshold <= n`
* <code>1 <= queries.length <= 10<sup>5</sup></code>
* `queries[i].length == 2`
* <code>1 <= a<sub>i</sub>, b<sub>i</sub> <= cities</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def areConnected(self, n: int, threshold: int, queries: List[List[int]]) -> List[bool]:
        if threshold == 0:
            return [True] * len(queries)

        parent = list(range(n + 1))

        for x in range(threshold + 1, n + 1):
            for z in range(2, int(sqrt(x)) + 1):
                if x // z <= threshold:
                    break
                if x % z == 0:
                    if z > threshold:
                        while parent[z] != parent[parent[z]]:
                            parent[z] = parent[parent[z]]
                        parent[parent[z]] = x
                    z = x // z
                    while parent[z] != parent[parent[z]]:
                        parent[z] = parent[parent[z]]
                    parent[parent[z]] = x

        for x in range(threshold + 1, n + 1):
            while parent[x] != parent[parent[x]]:
                parent[x] = parent[parent[x]]

        return [parent[a] == parent[b] for a, b in queries]
```
