# 2169. 得到 0 的操作数
给你两个 **非负** 整数 `num1` 和 `num2` 。

每一步 **操作** 中，如果 `num1 >= num2` ，你必须用 `num1` 减 `num2` ；否则，你必须用 `num2` 减 `num1` 。
* 例如，`num1 = 5` 且 `num2 = 4` ，应该用 `num1` 减 `num2` ，因此，得到 `num1 = 1` 和 `num2 = 4` 。然而，如果 `num1 = 4`且 `num2 = 5` ，一步操作后，得到 `num1 = 4` 和 `num2 = 1` 。

返回使 `num1 = 0` 或 `num2 = 0` 的 **操作数** 。

#### 示例 1:
<pre>
<strong>输入:</strong> num1 = 2, num2 = 3
<strong>输出:</strong> 3
<strong>解释:</strong>
- 操作 1 ：num1 = 2 ，num2 = 3 。由于 num1 < num2 ，num2 减 num1 得到 num1 = 2 ，num2 = 3 - 2 = 1 。
- 操作 2 ：num1 = 2 ，num2 = 1 。由于 num1 > num2 ，num1 减 num2 。
- 操作 3 ：num1 = 1 ，num2 = 1 。由于 num1 == num2 ，num1 减 num2 。
此时 num1 = 0 ，num2 = 1 。由于 num1 == 0 ，不需要再执行任何操作。
所以总操作数是 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num1 = 10, num2 = 10
<strong>输出:</strong> 1
<strong>解释:</strong>
- 操作 1 ：num1 = 10 ，num2 = 10 。由于 num1 == num2 ，num1 减 num2 得到 num1 = 10 - 10 = 0 。
此时 num1 = 0 ，num2 = 10 。由于 num1 == 0 ，不需要再执行任何操作。
所以总操作数是 1 。
</pre>

#### 提示:
* <code>0 <= num1, num2 <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def countOperations(self, num1: int, num2: int) -> int:
        ret = 0

        while num1 != 0 and num2 != 0:
            if num1 >= num2:
                num1 -= num2
            else:
                num2 -= num1

            ret += 1

        return ret
```
