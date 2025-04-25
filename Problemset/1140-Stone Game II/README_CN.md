# 1140. 石子游戏 II
Alice 和 Bob 继续他们的石子游戏。许多堆石子 **排成一行**，每堆都有正整数颗石子 `piles[i]`。游戏以谁手中的石子最多来决出胜负。

Alice 和 Bob 轮流进行，Alice 先开始。最初，`M = 1`。

在每个玩家的回合中，该玩家可以拿走剩下的 **前** `X` 堆的所有石子，其中 `1 <= X <= 2M`。然后，令 `M = max(M, X)`。

游戏一直持续到所有石子都被拿走。

假设 Alice 和 Bob 都发挥出最佳水平，返回 Alice 可以得到的最大数量的石头。

#### 示例 1:
<pre>
<strong>输入:</strong> piles = [2,7,9,4,4]
<strong>输出:</strong> 10
<strong>解释:</strong> 如果一开始 Alice 取了一堆，Bob 取了两堆，然后 Alice 再取两堆。Alice 可以得到 2 + 4 + 4 = 10 堆。
如果 Alice 一开始拿走了两堆，那么 Bob 可以拿走剩下的三堆。在这种情况下，Alice 得到 2 + 7 = 9 堆。返回 10，因为它更大。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> piles = [1,2,3,4,5,100]
<strong>输出:</strong> 104
</pre>

#### 提示:
* `1 <= piles.length <= 100`
* <code>1 <= piles[i] <= 10<sup>4</sup></code>

## 题解 (Python)

### 1. 题解
```Python
from functools import cache


class Solution:
    def stoneGameII(self, piles: List[int]) -> int:
        @cache
        def subGame(m: int, i: int) -> int:
            if i == len(piles):
                return 0

            total = sum(piles[i:])

            return total - min(subGame(max(m, x), i + x) for x in range(1, min(2 * m, len(piles) - i) + 1))

        return subGame(1, 0)
```
