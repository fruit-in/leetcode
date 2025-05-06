# 1201. 丑数 III
丑数是可以被 `a` 或 `b` 或 `c` 整除的 **正整数** 。

给你四个整数：`n` 、`a` 、`b` 、`c` ，请你设计一个算法来找出第 `n` 个丑数。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 3, a = 2, b = 3, c = 5
<strong>输出:</strong> 4
<strong>解释:</strong> 丑数序列为 2, 3, 4, 5, 6, 8, 9, 10... 其中第 3 个是 4。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 4, a = 2, b = 3, c = 4
<strong>输出:</strong> 6
<strong>解释:</strong> 丑数序列为 2, 3, 4, 6, 8, 9, 10, 12... 其中第 4 个是 6。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 5, a = 2, b = 11, c = 13
<strong>输出:</strong> 10
<strong>解释:</strong> 丑数序列为 2, 4, 6, 8, 10, 11, 12, 13... 其中第 5 个是 10。
</pre>

#### 提示:
* <code>1 <= n, a, b, c <= 10<sup>9</sup></code>
* <code>1 <= a * b * c <= 10<sup>18</sup></code>
* 本题结果在 <code>[1, 2 * 10<sup>9</sup>]</code> 的范围内

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def nthUglyNumber(self, n: int, a: int, b: int, c: int) -> int:
        def leUglyCount(x: int) -> int:
            return x // a + x // b + x // c - x // d - x // e - x // f + x // g

        d, e, f, g = math.lcm(a, b), math.lcm(
            b, c),  math.lcm(a, c), math.lcm(a, b, c)

        return bisect.bisect_left(range(2000000000), n, key=leUglyCount)
```
