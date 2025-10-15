# 906. 超级回文数
如果一个正整数自身是回文数，而且它也是一个回文数的平方，那么我们称这个数为 **超级回文数** 。

现在，给你两个以字符串形式表示的正整数 `left` 和 `right`  ，统计并返回区间 `[left, right]` 中的 **超级回文数** 的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> left = "4", right = "1000"
<strong>输出:</strong> 4
<strong>解释:</strong> 4、9、121 和 484 都是超级回文数。
注意 676 不是超级回文数：26 * 26 = 676 ，但是 26 不是回文数。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> left = "1", right = "2"
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= left.length, right.length <= 18`
* `left` 和 `right` 仅由数字（0 - 9）组成。
* `left` 和 `right` 不含前导零。
* `left` 和 `right` 表示的整数在区间 <code>[1, 10<sup>18</sup> - 1]</code> 内。
* `left` 小于等于 `right` 。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def superpalindromesInRange(self, left: str, right: str) -> int:
        left = int(left)
        right = int(right)
        ret = 0

        for leftpart in range(1, right + 1):
            s = str(leftpart)
            palindrome = int(s + s[::-1])
            superpalindrome = palindrome * palindrome
            if left <= superpalindrome <= right and str(superpalindrome) == str(superpalindrome)[::-1]:
                ret += 1
            elif superpalindrome < left:
                continue
            palindrome = int(s + s[-2::-1])
            superpalindrome = palindrome * palindrome
            if left <= superpalindrome <= right and str(superpalindrome) == str(superpalindrome)[::-1]:
                ret += 1
            elif superpalindrome > right:
                break

        return ret
```
