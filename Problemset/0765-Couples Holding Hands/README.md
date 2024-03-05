# 765. Couples Holding Hands
There are `n` couples sitting in `2n` seats arranged in a row and want to hold hands.

The people and seats are represented by an integer array `row` where `row[i]` is the ID of the person sitting in the <code>i<sup>th</sup></code> seat. The couples are numbered in order, the first couple being `(0, 1)`, the second couple being `(2, 3)`, and so on with the last couple being `(2n - 2, 2n - 1)`.

Return *the minimum number of swaps so that every couple is sitting side by side*. A swap consists of choosing any two people, then they stand up and switch seats.

#### Example 1:
<pre>
<strong>Input:</strong> row = [0,2,1,3]
<strong>Output:</strong> 1
<strong>Explanation:</strong> We only need to swap the second (row[1]) and third (row[2]) person.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> row = [3,2,0,1]
<strong>Output:</strong> 0
<strong>Explanation:</strong> All couples are already seated side by side.
</pre>

#### Constraints:
* `2n == row.length`
* `2 <= n <= 30`
* `n` is even.
* `0 <= row[i] < 2n`
* All the elements of `row` are **unique**.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def minSwapsCouples(self, row: List[int]) -> int:
        row = [person // 2 for person in row]
        parent = list(range(len(row) // 2))
        groups = {}

        for i in range(0, len(row), 2):
            if row[i] == row[i + 1]:
                continue

            while parent[parent[row[i]]] != parent[row[i]]:
                parent[row[i]] = parent[parent[row[i]]]
            while parent[parent[row[i + 1]]] != parent[row[i + 1]]:
                parent[row[i + 1]] = parent[parent[row[i + 1]]]

            if parent[row[i]] < parent[row[i + 1]]:
                parent[parent[row[i + 1]]] = parent[row[i]]
            else:
                parent[parent[row[i]]] = parent[row[i + 1]]

        for person in parent:
            while parent[parent[person]] != parent[person]:
                parent[person] = parent[parent[person]]

            groups[parent[person]] = groups.get(parent[person], 0) + 1

        return sum(x - 1 for x in groups.values())
```
