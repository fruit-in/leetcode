# 1722. Minimize Hamming Distance After Swap Operations
You are given two integer arrays, `source` and `target`, both of length `n`. You are also given an array `allowedSwaps` where each <code>allowedSwaps[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that you are allowed to swap the elements at index <code>a<sub>i</sub></code> and index <code>b<sub>i</sub></code> (**0-indexed**) of array `source`. Note that you can swap elements at a specific pair of indices **multiple** times and in **any** order.

The **Hamming distance** of two arrays of the same length, `source` and `target`, is the number of positions where the elements are different. Formally, it is the number of indices `i` for `0 <= i <= n-1` where `source[i] != target[i]` (**0-indexed**).

Return *the **minimum Hamming distance** of* `source` *and* `target` *after performing **any** amount of swap operations on array* `source`.

#### Example 1:
<pre>
<strong>Input:</strong> source = [1,2,3,4], target = [2,1,4,5], allowedSwaps = [[0,1],[2,3]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> source can be transformed the following way:
- Swap indices 0 and 1: source = [2,1,3,4]
- Swap indices 2 and 3: source = [2,1,4,3]
The Hamming distance of source and target is 1 as they differ in 1 position: index 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> source = [1,2,3,4], target = [1,3,2,4], allowedSwaps = []
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are no allowed swaps.
The Hamming distance of source and target is 2 as they differ in 2 positions: index 1 and index 2.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> source = [5,1,2,4,3], target = [1,5,4,2,3], allowedSwaps = [[0,4],[4,2],[1,3],[1,4]]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `n == source.length == target.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= source[i], target[i] <= 10<sup>5</sup></code>
* <code>0 <= allowedSwaps.length <= 10<sup>5</sup></code>
* `allowedSwaps[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> <= n - 1</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def minimumHammingDistance(self, source: List[int], target: List[int], allowedSwaps: List[List[int]]) -> int:
        n = len(source)
        parent = list(range(n))
        groups = {}
        ret = n

        for a, b in allowedSwaps:
            while parent[a] != parent[parent[a]]:
                parent[a] = parent[parent[a]]
            while parent[b] != parent[parent[b]]:
                parent[b] = parent[parent[b]]

            if parent[a] < parent[b]:
                parent[parent[b]] = parent[a]
            else:
                parent[parent[a]] = parent[b]

        for i in range(n):
            while parent[i] != parent[parent[i]]:
                parent[i] = parent[parent[i]]

            if parent[i] not in groups:
                groups[parent[i]] = {}
            if source[i] not in groups[parent[i]]:
                groups[parent[i]][source[i]] = 0
            groups[parent[i]][source[i]] += 1

        for i in range(n):
            if groups[parent[i]].get(target[i], 0) > 0:
                groups[parent[i]][target[i]] -= 1
                ret -= 1

        return ret
```
