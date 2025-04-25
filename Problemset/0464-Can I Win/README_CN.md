# 464. 我能赢吗
在 "100 game" 这个游戏中，两名玩家轮流选择从 `1` 到 `10` 的任意整数，累计整数和，先使得累计整数和 **达到或超过**  100 的玩家，即为胜者。

如果我们将游戏规则改为 “玩家 **不能** 重复使用整数” 呢？

例如，两个玩家可以轮流从公共整数池中抽取从 1 到 15 的整数（不放回），直到累计整数和 >= 100。

给定两个整数 `maxChoosableInteger` （整数池中可选择的最大数）和 `desiredTotal`（累计和），若先出手的玩家能稳赢则返回 `true` ，否则返回 `false` 。假设两位玩家游戏时都表现 **最佳** 。

#### 示例 1:
<pre>
<strong>输入:</strong> maxChoosableInteger = 10, desiredTotal = 11
<strong>输出:</strong> false
<strong>解释:</strong>
无论第一个玩家选择哪个整数，他都会失败。
第一个玩家可以选择从 1 到 10 的整数。
如果第一个玩家选择 1，那么第二个玩家只能选择从 2 到 10 的整数。
第二个玩家可以通过选择整数 10（那么累积和为 11 >= desiredTotal），从而取得胜利.
同样地，第一个玩家选择任意其他整数，第二个玩家都会赢。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> maxChoosableInteger = 10, desiredTotal = 0
<strong>输出:</strong> true
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> maxChoosableInteger = 10, desiredTotal = 1
<strong>输出:</strong> true
</pre>

#### 提示:
* `1 <= maxChoosableInteger <= 20`
* `0 <= desiredTotal <= 300`

## 题解 (Python)

### 1. 题解
```Python
from functools import cache


class Solution:
    def canIWin(self, maxChoosableInteger: int, desiredTotal: int) -> bool:
        @cache
        def canIWinWithUsed(usedmask: int) -> bool:
            total = sum(i + 1 for i in range(maxChoosableInteger)
                        if (usedmask >> i) & 1 == 1)

            for i in range(maxChoosableInteger):
                if (usedmask >> i) & 1 == 0:
                    if total + i + 1 >= desiredTotal or not canIWinWithUsed(usedmask | (1 << i)):
                        return True

            return False

        return sum(range(1, maxChoosableInteger + 1)) >= desiredTotal and canIWinWithUsed(0)
```
