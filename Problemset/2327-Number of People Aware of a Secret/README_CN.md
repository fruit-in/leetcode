# 2327. 知道秘密的人数
在第 `1` 天，有一个人发现了一个秘密。

给你一个整数 `delay` ，表示每个人会在发现秘密后的 `delay` 天之后，**每天** 给一个新的人 **分享** 秘密。同时给你一个整数 `forget` ，表示每个人在发现秘密 `forget` 天之后会 **忘记** 这个秘密。一个人 **不能** 在忘记秘密那一天及之后的日子里分享秘密。

给你一个整数 `n` ，请你返回在第 `n` 天结束时，知道秘密的人数。由于答案可能会很大，请你将结果对 <code>10<sup>9</sup> + 7</code> **取余** 后返回。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 6, delay = 2, forget = 4
<strong>输出:</strong> 5
<strong>解释:</strong>
第 1 天：假设第一个人叫 A 。（一个人知道秘密）
第 2 天：A 是唯一一个知道秘密的人。（一个人知道秘密）
第 3 天：A 把秘密分享给 B 。（两个人知道秘密）
第 4 天：A 把秘密分享给一个新的人 C 。（三个人知道秘密）
第 5 天：A 忘记了秘密，B 把秘密分享给一个新的人 D 。（三个人知道秘密）
第 6 天：B 把秘密分享给 E，C 把秘密分享给 F 。（五个人知道秘密）
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 4, delay = 1, forget = 3
<strong>输出:</strong> 6
<strong>解释:</strong>
第 1 天：第一个知道秘密的人为 A 。（一个人知道秘密）
第 2 天：A 把秘密分享给 B 。（两个人知道秘密）
第 3 天：A 和 B 把秘密分享给 2 个新的人 C 和 D 。（四个人知道秘密）
第 4 天：A 忘记了秘密，B、C、D 分别分享给 3 个新的人。（六个人知道秘密）
</pre>

#### 提示:
* `2 <= n <= 1000`
* `1 <= delay < forget <= n`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def peopleAwareOfSecret(self, n: int, delay: int, forget: int) -> int:
        dp = [[0, 0, 0] for _ in range(n + 2)]
        dp[1 + forget][0] = 1
        dp[1 + delay][1] = 1
        dp[1][2] = 1

        for i in range(2, n + 1):
            if i > 1 + forget:
                dp[i][0] = dp[i - forget][1]
            if i > 1 + delay:
                dp[i][1] = dp[i - 1][1] - dp[i][0] + dp[i - delay][1]
            dp[i][2] = dp[i - 1][2] - dp[i][0] + dp[i][1]

        return dp[n][2] % 1000000007
```
