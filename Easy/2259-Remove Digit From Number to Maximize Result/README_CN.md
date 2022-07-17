# 2259. 移除指定数字得到的最大结果
给你一个表示某个正整数的字符串 `number` 和一个字符 `digit` 。

从 `number` 中 **恰好** 移除 一个 等于 `digit` 的字符后，找出并返回按 **十进制** 表示 **最大** 的结果字符串。生成的测试用例满足 `digit` 在 `number` 中出现至少一次。

#### 示例 1:
<pre>
<strong>输入:</strong> number = "123", digit = "3"
<strong>输出:</strong> "12"
<strong>解释:</strong> "123" 中只有一个 '3' ，在移除 '3' 之后，结果为 "12" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> number = "1231", digit = "1"
<strong>输出:</strong> "231"
<strong>解释:</strong> 可以移除第一个 '1' 得到 "231" 或者移除第二个 '1' 得到 "123" 。
由于 231 > 123 ，返回 "231" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> number = "551", digit = "5"
<strong>输出:</strong> "51"
<strong>解释:</strong> 可以从 "551" 中移除第一个或者第二个 '5' 。
两种方案的结果都是 "51" 。
</pre>

#### 提示:
* `2 <= number.length <= 100`
* `number` 由数字 `'1'` 到 `'9'` 组成
* `digit` 是 `'1'` 到 `'9'` 中的一个数字
* `digit` 在 `number` 中出现至少一次

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def removeDigit(self, number: str, digit: str) -> str:
        i = 0

        for j in range(len(number)):
            if number[j] == digit:
                i = j
                if j + 1 < len(number) and number[j] < number[j + 1]:
                    break

        return number[:i] + number[i + 1:]
```
