# 2646. Minimize the Total Price of the Trips
There exists an undirected and unrooted tree with `n` nodes indexed from `0` to `n - 1`. You are given the integer `n` and a 2D integer array `edges` of length `n - 1`, where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that there is an edge between nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code> in the tree.

Each node has an associated price. You are given an integer array `price`, where `price[i]` is the price of the <code>i<sup>th</sup></code> node.

The **price sum** of a given path is the sum of the prices of all nodes lying on that path.

Additionally, you are given a 2D integer array `trips`, where <code>trips[i] = [start<sub>i</sub>, end<sub>i</sub>]</code> indicates that you start the ith trip from the node <code>start<sub>i</sub></code> and travel to the node <code>end<sub>i</sub></code> by any path you like.

Before performing your first trip, you can choose some **non-adjacent** nodes and halve the prices.

Return *the minimum total price sum to perform all the given trips*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2023/03/16/diagram2.png)
<pre>
<strong>Input:</strong> n = 4, edges = [[0,1],[1,2],[1,3]], price = [2,2,10,6], trips = [[0,3],[2,1],[2,3]]
<strong>Output:</strong> 23
<strong>Explanation:</strong> The diagram above denotes the tree after rooting it at node 2. The first part shows the initial tree and the second part shows the tree after choosing nodes 0, 2, and 3, and making their price half.
For the 1st trip, we choose path [0,1,3]. The price sum of that path is 1 + 2 + 3 = 6.
For the 2nd trip, we choose path [2,1]. The price sum of that path is 2 + 5 = 7.
For the 3rd trip, we choose path [2,1,3]. The price sum of that path is 5 + 2 + 3 = 10.
The total price sum of all trips is 6 + 7 + 10 = 23.
It can be proven, that 23 is the minimum answer that we can achieve.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2023/03/16/diagram3.png)
<pre>
<strong>Input:</strong> n = 2, edges = [[0,1]], price = [2,2], trips = [[0,0]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The diagram above denotes the tree after rooting it at node 0. The first part shows the initial tree and the second part shows the tree after choosing node 0, and making its price half.
For the 1st trip, we choose path [0]. The price sum of that path is 1.
The total price sum of all trips is 1. It can be proven, that 1 is the minimum answer that we can achieve.
</pre>

#### Constraints:
* `1 <= n <= 50`
* `edges.length == n - 1`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> <= n - 1</code>
* `edges` represents a valid tree.
* `price.length == n`
* `price[i]` is an even integer.
* `1 <= price[i] <= 1000`
* `1 <= trips.length <= 100`
* <code>0 <= start<sub>i</sub>, end<sub>i</sub> <= n - 1</code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def minimumTotalPrice(self, n: int, edges: List[List[int]], price: List[int], trips: List[List[int]]) -> int:
        def searchAndCount(start: int, end: int, prev: int = -1) -> bool:
            if start == end:
                count[start] += 1
                return True

            for neighbor in neighbors[start]:
                if neighbor != prev and searchAndCount(neighbor, end, start):
                    count[start] += 1
                    return True

            return False

        @cache
        def calculatePrice(root: int, canhalve: bool, prev: int = -1) -> int:
            price1 = inf
            price2 = price[root] * count[root]

            if canhalve and count[root] > 0:
                price1 = price2 // 2

                for neighbor in neighbors[root]:
                    if neighbor != prev:
                        price1 += calculatePrice(neighbor, False, root)

            for neighbor in neighbors[root]:
                if neighbor != prev:
                    price2 += min(calculatePrice(neighbor, False, root),
                                  calculatePrice(neighbor, True, root))

            return min(price1, price2)

        neighbors = [[] for _ in range(n)]
        count = [0] * n

        for a, b in edges:
            neighbors[a].append(b)
            neighbors[b].append(a)

        for start, end in trips:
            searchAndCount(start, end)

        return calculatePrice(0, True)
```
