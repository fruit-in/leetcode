# 2457. 美丽整数的最小增量
给你两个正整数 `n` 和 `target` 。

如果某个整数每一位上的数字相加小于或等于 `target` ，则认为这个整数是一个 **美丽整数** 。

找出并返回满足 `n + x` 是 **美丽整数** 的最小非负整数 `x` 。生成的输入保证总可以使 `n` 变成一个美丽整数。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 16, target = 6
<strong>输出:</strong> 4
<strong>解释:</strong> 最初，n 是 16 ，且其每一位数字的和是 1 + 6 = 7 。在加 4 之后，n 变为 20 且每一位数字的和变成 2 + 0 = 2 。可以证明无法加上一个小于 4 的非负整数使 n 变成一个美丽整数。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 467, target = 6
<strong>输出:</strong> 33
<strong>解释:</strong> 最初，n 是 467 ，且其每一位数字的和是 4 + 6 + 7 = 17 。在加 33 之后，n 变为 500 且每一位数字的和变成 5 + 0 + 0 = 5 。可以证明无法加上一个小于 33 的非负整数使 n 变成一个美丽整数。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 1, target = 1
<strong>输出:</strong> 0
<strong>解释:</strong> 最初，n 是 1 ，且其每一位数字的和是 1 ，已经小于等于 target 。
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>12</sup></code>
* `1 <= target <= 150`
* 生成的输入保证总可以使 `n` 变成一个美丽整数。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def makeIntegerBeautiful(self, n: int, target: int) -> int:
        a = 1
        m = n

        while sum(int(ch) for ch in str(m)) > target:
            m += (10 - m // a % 10) * a
            a *= 10

        return m - n
```
