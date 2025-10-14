# 2376. 统计特殊整数
如果一个正整数每一个数位都是 **互不相同** 的，我们称它是 **特殊整数** 。

给你一个 正 整数 `n` ，请你返回区间 `[1, n]` 之间特殊整数的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 20
<strong>输出:</strong> 19
<strong>解释:</strong> 1 到 20 之间所有整数除了 11 以外都是特殊整数。所以总共有 19 个特殊整数。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 5
<strong>输出:</strong> 5
<strong>解释:</strong> 1 到 5 所有整数都是特殊整数。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 135
<strong>输出:</strong> 110
<strong>解释:</strong> 从 1 到 135 总共有 110 个整数是特殊整数。
不特殊的部分数字为：22 ，114 和 131 。
</pre>

#### 提示:
* <code>1 <= n <= 2 * 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def countSpecialNumbers(self, n: int) -> int:
        @cache
        def countRemaining(s: str, mask: int) -> int:
            if s == "":
                return 1

            ret = 0

            for i in range(int(s[0])):
                if i == 0 and mask == 0:
                    ret += 9 * sum(362880 // factorial(j)
                                   for j in range(11 - len(s), 10)) + 1
                elif (mask >> i) & 1 == 0:
                    remain = 9 - mask.bit_count()
                    ret += prod(range(remain, remain - len(s) + 1, -1))
            if (mask >> int(s[0])) & 1 == 0:
                ret += countRemaining(s[1:], mask | (1 << int(s[0])))

            return ret

        return countRemaining(str(n), 0) - 1
```
