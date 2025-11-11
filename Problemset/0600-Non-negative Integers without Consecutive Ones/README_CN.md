# 600. 不含连续1的非负整数
给定一个正整数 `n` ，请你统计在 `[0, n]` 范围的非负整数中，有多少个整数的二进制表示中不存在 **连续的 1** 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 5
<strong>输出:</strong> 5
<strong>解释:</strong>
下面列出范围在 [0, 5] 的非负整数与其对应的二进制表示：
0 : 0
1 : 1
2 : 10
3 : 11
4 : 100
5 : 101
其中，只有整数 3 违反规则（有两个连续的 1 ），其他 5 个满足规则。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> 2
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 2
<strong>输出:</strong> 3
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    @cache
    def findIntegers(self, n: int) -> int:
        if n < 4:
            return 3 if n == 3 else n + 1

        m = n.bit_length() - 2
        ret = self.findIntegers((1 << m) - 1) + \
            self.findIntegers((1 << (m - 1)) - 1)
        if n >> m == 0b10:
            ret += self.findIntegers(n & ((1 << m) - 1))
        else:
            ret += self.findIntegers((1 << m) - 1)

        return ret
```
