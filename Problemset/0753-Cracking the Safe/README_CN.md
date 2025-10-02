# 753. 破解保险箱
有一个需要密码才能打开的保险箱。密码是 `n` 位数, 密码的每一位都是范围 `[0, k - 1]` 中的一个数字。

保险箱有一种特殊的密码校验方法，你可以随意输入密码序列，保险箱会自动记住 **最后** `n` **位输入** ，如果匹配，则能够打开保险箱。

* 例如，正确的密码是 `"345"` ，并且你输入的是 `"012345"` ：
    * 输入 `0` 之后，最后 `3` 位输入是 `"0"` ，不正确。
    * 输入 `1` 之后，最后 `3` 位输入是 `"01"` ，不正确。
    * 输入 `2` 之后，最后 `3` 位输入是 `"012"` ，不正确。
    * 输入 `3` 之后，最后 `3` 位输入是 `"123"` ，不正确。
    * 输入 `4` 之后，最后 `3` 位输入是 `"234"` ，不正确。
    * 输入 `5` 之后，最后 `3` 位输入是 `"345"` ，正确，打开保险箱。

在只知道密码位数 `n` 和范围边界 `k` 的前提下，请你找出并返回确保在输入的 **某个时刻** 能够打开保险箱的任一 **最短** 密码序列 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 1, k = 2
<strong>输出:</strong> "10"
<strong>解释:</strong> 密码只有 1 位，所以输入每一位就可以。"01" 也能够确保打开保险箱。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 2, k = 2
<strong>输出:</strong> "01100"
<strong>解释:</strong> 对于每种可能的密码：
- "00" 从第 4 位开始输入。
- "01" 从第 1 位开始输入。
- "10" 从第 3 位开始输入。
- "11" 从第 2 位开始输入。
因此 "01100" 可以确保打开保险箱。"01100"、"10011" 和 "11001" 也可以确保打开保险箱。
</pre>

#### 提示:
* `1 <= n <= 4`
* `1 <= k <= 10`
* <code>1 <= k<sup>n</sup> <= 4096</code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def crackSafe(self, n: int, k: int) -> str:
        if n == 1:
            return ''.join(map(str, range(k)))

        unused = {}
        stack = []
        curr = "0" * (n - 1)
        ret = []

        for num in range(10 ** (n - 1)):
            num = str(num).zfill(n - 1)

            if all(int(digit) < k for digit in num):
                unused[num] = [num[1:] + str(digit) for digit in range(k)]

        while curr != "":
            if unused[curr]:
                stack.append(curr)
                curr = unused[curr].pop()
            else:
                ret.append(curr[-1])
                curr = stack.pop() if stack else ""

        ret.append("0" * (n - 2))

        return ''.join(ret)
```
