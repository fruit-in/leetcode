# 1925. 统计平方和三元组的数目
一个 **平方和三元组** `(a,b,c)` 指的是满足 <code>a<sup>2</sup> + b<sup>2</sup> = c<sup>2</sup></code> 的 **整数** 三元组 `a`，`b` 和 `c` 。

给你一个整数 `n` ，请你返回满足 `1 <= a, b, c <= n` 的 **平方和三元组** 的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 5
<strong>输出:</strong> 2
<strong>解释:</strong> 平方和三元组为 (3,4,5) 和 (4,3,5) 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 10
<strong>输出:</strong> 4
<strong>解释:</strong> 平方和三元组为 (3,4,5)，(4,3,5)，(6,8,10) 和 (8,6,10) 。
</pre>

#### 提示:
* `1 <= n <= 250`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def countTriples(self, n: int) -> int:
        ret = 0

        for a in range(1, n):
            for b in range(1, n):
                c = int((a * a + b * b) ** 0.5)
                if c <= n and c * c == a * a + b * b:
                    ret += 1

        return ret
```
