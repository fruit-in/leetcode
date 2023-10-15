# 564. Find the Closest Palindrome
Given a string `n` representing an integer, return *the closest integer (not including itself), which is a palindrome*. If there is a tie, return ***the smaller one***.

The closest is defined as the absolute difference minimized between two integers.

#### Example 1:
<pre>
<strong>Input:</strong> n = "123"
<strong>Output:</strong> "121"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = "1"
<strong>Output:</strong> "0"
<strong>Explanation:</strong> 0 and 2 are the closest palindromes but we return the smallest which is 0.
</pre>

#### Constraints:
* `1 <= n.length <= 18`
* `n` consists of only digits.
* `n` does not have leading zeros.
* `n` is representing an integer in the range <code>[1, 10<sup>18</sup> - 1]</code>.

## Solutions (Python)

### 1. Solution
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
