# 2266. 统计打字方案数
Alice 在给 Bob 用手机打字。数字到字母的 **对应** 如下图所示。

![](https://assets.leetcode.com/uploads/2022/03/15/1200px-telephone-keypad2svg.png)

为了 **打出** 一个字母，Alice 需要 **按** 对应字母 `i` 次，`i` 是该字母在这个按键上所处的位置。

* 比方说，为了按出字母 `'s'` ，Alice 需要按 `'7'` 四次。类似的， Alice 需要按 `'5'` 两次得到字母  `'k'` 。
* 注意，数字 `'0'` 和 `'1'` 不映射到任何字母，所以 Alice **不** 使用它们。

但是，由于传输的错误，Bob 没有收到 Alice 打字的字母信息，反而收到了 **按键的字符串信息** 。

* 比方说，Alice 发出的信息为 `"bob"` ，Bob 将收到字符串 `"2266622"` 。

给你一个字符串 `pressedKeys` ，表示 Bob 收到的字符串，请你返回 Alice **总共可能发出多少种文字信息** 。

由于答案可能很大，将它对 <code>10<sup>9</sup> + 7</code> **取余** 后返回。

#### 示例 1:
<pre>
<strong>输入:</strong> pressedKeys = "22233"
<strong>输出:</strong> 8
<strong>解释:</strong>
Alice 可能发出的文字信息包括：
"aaadd", "abdd", "badd", "cdd", "aaae", "abe", "bae" 和 "ce" 。
由于总共有 8 种可能的信息，所以我们返回 8 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> pressedKeys = "222222222222222222222222222222222222"
<strong>输出:</strong> 82876089
<strong>解释:</strong>
总共有 2082876103 种 Alice 可能发出的文字信息。
由于我们需要将答案对 10<sup>9</sup> + 7 取余，所以我们返回 2082876103 % (10<sup>9</sup> + 7) = 82876089 。
</pre>

#### 提示:
* <code>1 <= pressedKeys.length <= 10<sup>5</sup></code>
* `pressedKeys` 只包含数字 `'2'` 到 `'9'` 。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def countTexts(self, pressedKeys: str) -> int:
        digit = '0'
        count = 0
        max3 = [1, 1, 2]
        max4 = [1, 1, 2, 4]
        ret = 1

        for d in (pressedKeys + '0'):
            if d == digit:
                count += 1
            else:
                if digit in "79":
                    while count >= len(max4):
                        max4.append(sum(max4[-4:]))
                    ret *= max4[count]
                else:
                    while count >= len(max3):
                        max3.append(sum(max3[-3:]))
                    ret *= max3[count]

                digit = d
                count = 1

        return ret % 1_000_000_007
```
