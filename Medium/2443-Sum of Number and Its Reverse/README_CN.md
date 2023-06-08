# 2443. 反转之后的数字和
给你一个 **非负** 整数 `num` 。如果存在某个 **非负** 整数 `k` 满足 `k + reverse(k) = num`  ，则返回 `true` ；否则，返回 `false` 。

`reverse(k)` 表示 `k` 反转每个数位后得到的数字。

#### 示例 1:
<pre>
<strong>输入:</strong> num = 443
<strong>输出:</strong> true
<strong>解释:</strong> 172 + 271 = 443 ，所以返回 true 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = 63
<strong>输出:</strong> false
<strong>解释:</strong> 63 不能表示为非负整数及其反转后数字之和，返回 false 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> num = 181
<strong>输出:</strong> true
<strong>解释:</strong> 140 + 041 = 181 ，所以返回 true 。注意，反转后的数字可能包含前导零。
</pre>

#### 提示:
* <code>0 <= num <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def sumOfNumberAndReverse(self, num: int) -> bool:
        return any(k + int(str(k)[::-1]) == num for k in range(num + 1))
```
