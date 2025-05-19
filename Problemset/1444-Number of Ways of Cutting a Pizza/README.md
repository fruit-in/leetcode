# 1444. Number of Ways of Cutting a Pizza
Given a rectangular pizza represented as a `rows x cols` matrix containing the following characters: `'A'` (an apple) and `'.'` (empty cell) and given the integer `k`. You have to cut the pizza into `k` pieces using `k-1` cuts.

For each cut you choose the direction: vertical or horizontal, then you choose a cut position at the cell boundary and cut the pizza into two pieces. If you cut the pizza vertically, give the left part of the pizza to a person. If you cut the pizza horizontally, give the upper part of the pizza to a person. Give the last piece of pizza to the last person.

*Return the number of ways of cutting the pizza such that each piece contains **at least** one apple*. Since the answer can be a huge number, return this modulo 10^9 + 7.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/04/23/ways_to_cut_apple_1.png)
<pre>
<strong>Input:</strong> pizza = ["A..","AAA","..."], k = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> The figure above shows the three ways to cut the pizza. Note that pieces must contain at least one apple.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> pizza = ["A..","AA.","..."], k = 3
<strong>Output:</strong> 1
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> pizza = ["A..","A..","..."], k = 1
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= rows, cols <= 50`
* `rows == pizza.length`
* `cols == pizza[i].length`
* `1 <= k <= 10`
* `pizza` consists of characters `'A'` and `'.'` only.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def ways(self, pizza: List[str], k: int) -> int:
        @cache
        def subPizzaWays(r: int, c: int, k: int) -> int:
            if applecount[r][c] < k:
                return 0
            if k == 1:
                return 1

            ret = 0

            for i in range(r + 1, rows):
                if applecount[r][c] > applecount[i][c]:
                    ret = (ret + subPizzaWays(i, c, k - 1)) % 1000000007
            for i in range(c + 1, cols):
                if applecount[r][c] > applecount[r][i]:
                    ret = (ret + subPizzaWays(r, i, k - 1)) % 1000000007

            return ret

        rows = len(pizza)
        cols = len(pizza[0])
        applecount = [[0] * (cols) for _ in range(rows)]

        for r in range(rows - 1, -1, -1):
            for c in range(cols - 1, -1, -1):
                if pizza[r][c] == 'A':
                    applecount[r][c] = 1
                if r + 1 < rows:
                    applecount[r][c] += applecount[r + 1][c]
                if c + 1 < cols:
                    applecount[r][c] += applecount[r][c + 1]
                if r + 1 < rows and c + 1 < cols:
                    applecount[r][c] -= applecount[r + 1][c + 1]

        return subPizzaWays(0, 0, k)
```
