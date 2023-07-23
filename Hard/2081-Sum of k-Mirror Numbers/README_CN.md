# 2081. k 镜像数字的和
一个 **k 镜像数字** 指的是一个在十进制和 k 进制下从前往后读和从后往前读都一样的 **没有前导 0** 的 **正** 整数。

* 比方说，`9` 是一个 2 镜像数字。`9` 在十进制下为 `9` ，二进制下为 `1001` ，两者从前往后读和从后往前读都一样。
* 相反地，`4` 不是一个 2 镜像数字。`4` 在二进制下为 `100` ，从前往后和从后往前读不相同。

给你进制 `k` 和一个数字 `n` ，请你返回 k 镜像数字中 **最小** 的 `n` 个数 **之和** 。

#### 示例 1:
<pre>
<strong>输入:</strong> k = 2, n = 5
<strong>输出:</strong> 25
<strong>解释:</strong>
最小的 5 个 2 镜像数字和它们的二进制表示如下：
  十进制       二进制
    1          1
    3          11
    5          101
    7          111
    9          1001
它们的和为 1 + 3 + 5 + 7 + 9 = 25 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> k = 3, n = 7
<strong>输出:</strong> 499
<strong>解释:</strong>
7 个最小的 3 镜像数字和它们的三进制表示如下：
  十进制       三进制
    1          1
    2          2
    4          11
    8          22
    121        11111
    151        12121
    212        21212
它们的和为 1 + 2 + 4 + 8 + 121 + 151 + 212 = 499 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> k = 7, n = 17
<strong>输出:</strong> 20379000
<strong>解释:</strong> 17 个最小的 7 镜像数字分别为：
1, 2, 3, 4, 5, 6, 8, 121, 171, 242, 292, 16561, 65656, 2137312, 4602064, 6597956, 6958596
</pre>

#### 提示:
* `2 <= k <= 9`
* `1 <= n <= 30`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def kMirror(self, k: int, n: int) -> int:
        x = 1
        nums = []

        while True:
            for a in range(x, 10 * x):
                b = c = int(str(a) + str(a)[-2::-1])
                d = ''
                while c > 0:
                    d = str(c % k) + d
                    c //= k
                if d == d[::-1]:
                    nums.append(b)
                if len(nums) == n:
                    return sum(nums)
            for a in range(x, 10 * x):
                b = c = int(str(a) + str(a)[::-1])
                d = ''
                while c > 0:
                    d = str(c % k) + d
                    c //= k
                if str(d) == str(d)[::-1]:
                    nums.append(b)
                if len(nums) == n:
                    return sum(nums)

            x *= 10
```
