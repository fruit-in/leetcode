# 557. Reverse Words in a String III
Given a string, you need to reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.

#### Example 1:
<pre>
<strong>Input:</strong> "Let's take LeetCode contest"
<strong>Output:</strong> "s'teL ekat edoCteeL tsetnoc"
</pre>

**Note:** In the string, each word is separated by single space and there will not be any extra space in the string.

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def reverseWords(self, s: str) -> str:
        return ' '.join(word[::-1] for word in s.split())
```
