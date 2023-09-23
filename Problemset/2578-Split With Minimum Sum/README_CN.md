# 2578. 最小和分割
给你一个正整数 `num` ，请你将它分割成两个非负整数 `num1` 和 `num2` ，满足：

* `num1` 和 `num2` 直接连起来，得到 `num` 各数位的一个排列。
    * 换句话说，`num1` 和 `num2` 中所有数字出现的次数之和等于 `num` 中所有数字出现的次数。
* `num1` 和 `num2` 可以包含前导 0 。

请你返回 `num1` 和 `num2` 可以得到的和的 **最小** 值。

**注意：**

* `num` 保证没有前导 0 。
* `num1` 和 `num2` 中数位顺序可以与 `num` 中数位顺序不同。

#### 示例 1:
<pre>
<strong>输入:</strong> num = 4325
<strong>输出:</strong> 59
<strong>解释:</strong> 我们可以将 4325 分割成 num1 = 24 和 num2 = 35 ，和为 59 ，59 是最小和。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = 687
<strong>输出:</strong> 75
<strong>解释:</strong> 我们可以将 687 分割成 num1 = 68 和 num2 = 7 ，和为最优值 75 。
</pre>

#### 提示:
* <code>10 <= num <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def splitNum(self, num: int) -> int:
        num = sorted(str(num))
        num1 = int(''.join(num[::2]))
        num2 = int(''.join(num[1::2]))

        return num1 + num2
```
