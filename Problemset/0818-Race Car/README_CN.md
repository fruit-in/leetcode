# 818. 赛车
你的赛车可以从位置 `0` 开始，并且速度为 `+1` ，在一条无限长的数轴上行驶。赛车也可以向负方向行驶。赛车可以按照由加速指令 `'A'` 和倒车指令 `'R'` 组成的指令序列自动行驶。
* 当收到指令 `'A'` 时，赛车这样行驶：
    * `position += speed`
    * `speed *= 2`
* 当收到指令 `'R'` 时，赛车这样行驶：
    * 如果速度为正数，那么`speed = -1`
    * 否则 `speed = 1`
    * 当前所处位置不变。

例如，在执行指令 `"AAR"` 后，赛车位置变化为 `0 --> 1 --> 3 --> 3` ，速度变化为 `1 --> 2 --> 4 --> -1` 。

给你一个目标位置 `target` ，返回能到达目标位置的最短指令序列的长度。

#### 示例 1:
<pre>
<strong>输入:</strong> target = 3
<strong>输出:</strong> 2
<strong>解释:</strong>
最短指令序列是 "AA" 。
位置变化 0 --> 1 --> 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> target = 6
<strong>输出:</strong> 5
<strong>解释:</strong>
最短指令序列是 "AAARA" 。
位置变化 0 --> 1 --> 3 --> 7 --> 7 --> 6 。
</pre>

#### 提示:
* <code>1 <= target <= 10<sup>4</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    aseqpositions = [(1 << i) - 1 for i in range(15)]

    @cache
    def racecar(self, target: int, reverse: bool = False) -> int:
        ret = inf

        for i in range(15):
            if not reverse and self.aseqpositions[i] == target:
                return i

            for j in range(i - 1, -1, -1):
                position = self.aseqpositions[i] - self.aseqpositions[j]

                if position < target:
                    ret = min(ret, i + j + 2 +
                              self.racecar(target - position, reverse))
                elif position == target:
                    ret = min(ret, i + j + 1)
                elif position - target < target:
                    ret = min(ret, i + j + 2 +
                              self.racecar(position - target, not reverse))
                else:
                    break

        return ret
```
