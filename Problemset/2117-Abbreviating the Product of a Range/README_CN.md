# 2117. 一个区间内所有数乘积的缩写
给你两个正整数 `left` 和 `right` ，满足 `left <= right` 。请你计算 **闭区间** `[left, right]` 中所有整数的 **乘积** 。

由于乘积可能非常大，你需要将它按照以下步骤 **缩写** ：
1. 统计乘积中 **后缀** 0 的数目，并 **移除** 这些 0 ，将这个数目记为 `C` 。
    * 比方说，`1000` 中有 `3` 个后缀 0 ，`546` 中没有后缀 0 。
2. 将乘积中剩余数字的位数记为 `d` 。如果 `d > 10` ，那么将乘积表示为 `<pre>...<suf>` 的形式，其中 `<pre>` 表示乘积最 **开始** 的 `5` 个数位，`<suf>` 表示删除后缀 0 **之后** 结尾的 `5` 个数位。如果 `d <= 10` ，我们不对它做修改。
    * 比方说，我们将 `1234567654321` 表示为 `12345...54321` ，但是 `1234567` 仍然表示为 `1234567` 。
3. 最后，将乘积表示为 **字符串** `"<pre>...<suf>eC"` 。
    * 比方说，`12345678987600000` 被表示为 `"12345...89876e5"` 。

请你返回一个字符串，表示 **闭区间** `[left, right]` 中所有整数 **乘积** 的 **缩写** 。

#### 示例 1:
<pre>
<strong>输入:</strong> left = 1, right = 4
<strong>输出:</strong> "24e0"
<strong>解释:</strong>
乘积为 1 × 2 × 3 × 4 = 24 。
由于没有后缀 0 ，所以 24 保持不变，缩写的结尾为 "e0" 。
因为乘积的结果是 2 位数，小于 10 ，所欲我们不进一步将它缩写。
所以，最终将乘积表示为 "24e0" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> left = 2, right = 11
<strong>输出:</strong> "399168e2"
<strong>解释:</strong> 乘积为 39916800 。
有 2 个后缀 0 ，删除后得到 399168 。缩写的结尾为 "e2" 。
删除后缀 0 后是 6 位数，不需要进一步缩写。
所以，最终将乘积表示为 "399168e2" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> left = 371, right = 375
<strong>输出:</strong> "7219856259e3"
<strong>解释:</strong> 乘积为 7219856259000 。
</pre>

#### 提示:
* <code>1 <= left <= right <= 10<sup>4</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def abbreviateProduct(self, left: int, right: int) -> str:
        pre = 0
        suf = 1
        ec = 0
        ret = ""

        for x in range(left, right + 1):
            pre *= x
            suf *= x

            while suf % 10 == 0:
                suf //= 10
                ec += 1

            if pre == 0 and suf > 9999999999999999999999999999:
                pre = int(str(suf)[:17])
                suf %= 100000000000
            elif pre > 0 and suf > 99999999999:
                suf %= 100000000000

            while pre > 99999999999999999:
                pre //= 10

        if pre == 0 and suf > 9999999999:
            pre = int(str(suf)[:5])
            suf = int(str(suf)[-5:])

        if pre == 0:
            return str(suf) + "e" + str(ec)
        else:
            return str(pre)[:5] + "..." + "{:05}".format(suf)[-5:] + "e" + str(ec)
```
