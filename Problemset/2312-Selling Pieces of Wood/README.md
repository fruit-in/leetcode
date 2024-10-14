# 2312. Selling Pieces of Wood
You are given two integers `m` and `n` that represent the height and width of a rectangular piece of wood. You are also given a 2D integer array `prices`, where <code>prices[i] = [h<sub>i</sub>, w<sub>i</sub>, price<sub>i</sub>]</code> indicates you can sell a rectangular piece of wood of height <code>h<sub>i</sub></code> and width <code>w<sub>i</sub></code> for <code>price<sub>i</sub></code> dollars.

To cut a piece of wood, you must make a vertical or horizontal cut across the **entire** height or width of the piece to split it into two smaller pieces. After cutting a piece of wood into some number of smaller pieces, you can sell pieces according to `prices`. You may sell multiple pieces of the same shape, and you do not have to sell all the shapes. The grain of the wood makes a difference, so you **cannot** rotate a piece to swap its height and width.

Return *the **maximum** money you can earn after cutting an* `m x n` *piece of wood*.

Note that you can cut the piece of wood as many times as you want.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/04/27/ex1.png)
<pre>
<strong>Input:</strong> m = 3, n = 5, prices = [[1,4,2],[2,2,7],[2,1,3]]
<strong>Output:</strong> 19
<strong>Explanation:</strong> The diagram above shows a possible scenario. It consists of:
- 2 pieces of wood shaped 2 x 2, selling for a price of 2 * 7 = 14.
- 1 piece of wood shaped 2 x 1, selling for a price of 1 * 3 = 3.
- 1 piece of wood shaped 1 x 4, selling for a price of 1 * 2 = 2.
This obtains a total of 14 + 3 + 2 = 19 money earned.
It can be shown that 19 is the maximum amount of money that can be earned.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/04/27/ex2new.png)
<pre>
<strong>Input:</strong> m = 4, n = 6, prices = [[3,2,10],[1,4,2],[4,1,3]]
<strong>Output:</strong> 32
<strong>Explanation:</strong> The diagram above shows a possible scenario. It consists of:
- 3 pieces of wood shaped 3 x 2, selling for a price of 3 * 10 = 30.
- 1 piece of wood shaped 1 x 4, selling for a price of 1 * 2 = 2.
This obtains a total of 30 + 2 = 32 money earned.
It can be shown that 32 is the maximum amount of money that can be earned.
Notice that we cannot rotate the 1 x 4 piece of wood to obtain a 4 x 1 piece of wood.
</pre>

#### Constraints:
* `1 <= m, n <= 200`
* <code>1 <= prices.length <= 2 * 10<sup>4</sup></code>
* `prices[i].length == 3`
* <code>1 <= h<sub>i</sub> <= m</code>
* <code>1 <= w<sub>i</sub> <= n</code>
* <code>1 <= price<sub>i</sub> <= 10<sup>6</sup></code>
* All the shapes of wood <code>(h<sub>i</sub>, w<sub>i</sub>)</code> are pairwise **distinct**.

## Solutions (Python)

### 1. Solution
```Python
from functools import cache


class Solution:
    def sellingWood(self, m: int, n: int, prices: List[List[int]]) -> int:
        prices = {(h, w): price for h, w, price in prices}

        @cache
        def maxSelling(m: int, n: int) -> int:
            ret = prices.get((m, n), 0)

            for h in range(1, m):
                ret = max(ret, maxSelling(h, n) + maxSelling(m - h, n))
            for w in range(1, n):
                ret = max(ret, maxSelling(m, w) + maxSelling(m, n - w))

            return ret

        return maxSelling(m, n)
```
