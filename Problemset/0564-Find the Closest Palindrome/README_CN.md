# 564. 寻找最近的回文数
给定一个表示整数的字符串 `n` ，返回与它最近的回文整数（不包括自身）。如果不止一个，返回较小的那个。

“最近的”定义为两个整数**差的绝对值**最小。

#### 示例 1:
<pre>
<strong>输入:</strong> n = "123"
<strong>输出:</strong> "121"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = "1"
<strong>输出:</strong> "0"
<strong>解释:</strong> 0 和 2是最近的回文，但我们返回最小的，也就是 0。
</pre>

#### 提示:
* `1 <= n.length <= 18`
* `n` 只由数字组成
* `n` 不含前导 0
* `n` 代表在 <code>[1, 10<sup>18</sup> - 1]</code> 范围内的整数

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def nearestPalindromic(self, n: str) -> str:
        nums = []
        nums.append(str(int(n[:(len(n) + 1) // 2])))
        nums.append(str(int(n[:(len(n) + 1) // 2]) - 1))
        nums.append(str(int(n[:(len(n) + 1) // 2]) + 1))
        nums[0] += nums[0][:len(n) // 2][::-1]
        nums[1] += nums[1][:len(n) // 2][::-1]
        nums[2] += nums[2][:len(n) // 2][::-1]
        nums.append("9" * max(len(n) - 1, 1))
        nums.sort(key=lambda x: (x == n, abs(int(x) - int(n)), int(x)))

        return nums[0]
```
