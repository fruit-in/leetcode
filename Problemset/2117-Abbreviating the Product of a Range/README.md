# 2117. Abbreviating the Product of a Range
You are given two positive integers `left` and `right` with `left <= right`. Calculate the **product** of all integers in the **inclusive** range `[left, right]`.

Since the product may be very large, you will **abbreviate** it following these steps:
1. Count all **trailing** zeros in the product and **remove** them. Let us denote this count as `C`.
    * For example, there are `3` trailing zeros in `1000`, and there are `0` trailing zeros in `546`.
2. Denote the remaining number of digits in the product as `d`. If `d > 10`, then express the product as `<pre>...<suf>` where `<pre>` denotes the **first** `5` digits of the product, and `<suf>` denotes the **last** `5` digits of the product **after** removing all trailing zeros. If `d <= 10`, we keep it unchanged.
    * For example, we express `1234567654321` as `12345...54321`, but `1234567` is represented as `1234567`.
3. Finally, represent the product as a **string** "`<pre>...<suf>eC`".
    * For example, `12345678987600000` will be represented as `"12345...89876e5"`.

Return *a string denoting the **abbreviated product** of all integers in the **inclusive** range* `[left, right]`.

#### Example 1:
<pre>
<strong>Input:</strong> left = 1, right = 4
<strong>Output:</strong> "24e0"
<strong>Explanation:</strong> The product is 1 × 2 × 3 × 4 = 24.
There are no trailing zeros, so 24 remains the same. The abbreviation will end with "e0".
Since the number of digits is 2, which is less than 10, we do not have to abbreviate it further.
Thus, the final representation is "24e0".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> left = 2, right = 11
<strong>Output:</strong> "399168e2"
<strong>Explanation:</strong> The product is 39916800.
There are 2 trailing zeros, which we remove to get 399168. The abbreviation will end with "e2".
The number of digits after removing the trailing zeros is 6, so we do not abbreviate it further.
Hence, the abbreviated product is "399168e2".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> left = 371, right = 375
<strong>Output:</strong> "7219856259e3"
<strong>Explanation:</strong> The product is 7219856259000.
</pre>

#### Constraints:
* <code>1 <= left <= right <= 10<sup>4</sup></code>

## Solutions (Python)

### 1. Solution
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
