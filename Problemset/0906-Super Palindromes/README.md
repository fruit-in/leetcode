# 906. Super Palindromes
Let's say a positive integer is a **super-palindrome** if it is a palindrome, and it is also the square of a palindrome.

Given two positive integers `left` and `right` represented as strings, return *the number of **super-palindromes** integers in the inclusive range* `[left, right]`.

#### Example 1:
<pre>
<strong>Input:</strong> left = "4", right = "1000"
<strong>Output:</strong> 4
<strong>Explanation:</strong> 4, 9, 121, and 484 are superpalindromes.
Note that 676 is not a superpalindrome: 26 * 26 = 676, but 26 is not a palindrome.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> left = "1", right = "2"
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= left.length, right.length <= 18`
* `left` and `right` consist of only digits.
* `left` and `right` cannot have leading zeros.
* `left` and `right` represent integers in the range <code>[1, 10<sup>18</sup> - 1]</code>.
* `left` is less than or equal to `right`.

## Solutions (Python)

### 1. Solution
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
