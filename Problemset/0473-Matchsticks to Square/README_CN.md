# 473. 火柴拼正方形
你将得到一个整数数组 `matchsticks` ，其中 `matchsticks[i]` 是第 `i` 个火柴棒的长度。你要用 **所有的火柴棍** 拼成一个正方形。你 **不能折断** 任何一根火柴棒，但你可以把它们连在一起，而且每根火柴棒必须 **使用一次** 。

如果你能使这个正方形，则返回 `true` ，否则返回 `false` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/04/09/matchsticks1-grid.jpg)
<pre>
<strong>输入:</strong> matchsticks = [1,1,2,2,2]
<strong>输出:</strong> true
<strong>解释:</strong> 能拼成一个边长为2的正方形，每边两根火柴。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> matchsticks = [3,3,3,3,4]
<strong>输出:</strong> false
<strong>解释:</strong> 不能用所有火柴拼成一个正方形。
</pre>

#### 提示:
* `1 <= matchsticks.length <= 15`
* <code>1 <= matchsticks[i] <= 10<sup>8</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def makesquare(self, matchsticks: List[int]) -> bool:
        perimeter = sum(matchsticks)

        if perimeter % 4 != 0:
            return False

        combs = {(0, 0, 0, 0)}

        for stick in matchsticks:
            newcombs = set()

            for comb in combs:
                for i in range(4):
                    if comb[i] + stick <= perimeter // 4:
                        newcomb = list(comb)
                        newcomb[i] += stick
                        newcombs.add(tuple(sorted(newcomb)))

            combs = newcombs

        return len(combs) > 0
```
