# 2000. Reverse Prefix of Word
Given a **0-indexed** string `word` and a character `ch`, **reverse** the segment of `word` that starts at index `0` and ends at the index of the **first occurrence** of `ch` (**inclusive**). If the character `ch` does not exist in `word`, do nothing.
* For example, if `word = "abcdefd"` and `ch = "d"`, then you should **reverse** the segment that starts at `0` and ends at `3` (**inclusive**). The resulting string will be `"dcbaefd"`.

Return *the resulting string*.

#### Example 1:
<pre>
<strong>Input:</strong> word = "abcdefd", ch = "d"
<strong>Output:</strong> "dcbaefd"
<strong>Explanation:</strong> The first occurrence of "d" is at index 3.
Reverse the part of word from 0 to 3 (inclusive), the resulting string is "dcbaefd".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> word = "xyxzxe", ch = "z"
<strong>Output:</strong> "zxyxxe"
<strong>Explanation:</strong> The first and only occurrence of "z" is at index 3.
Reverse the part of word from 0 to 3 (inclusive), the resulting string is "zxyxxe".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> word = "abcd", ch = "z"
<strong>Output:</strong> "abcd"
<strong>Explanation:</strong> "z" does not exist in word.
You should not do any reverse operation, the resulting string is "abcd".
</pre>

#### Constraints:
* `1 <= word.length <= 250`
* `word` consists of lowercase English letters.
* `ch` is a lowercase English letter.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def reversePrefix(self, word: str, ch: str) -> str:
        i = word.find(ch)

        return word[:i + 1][::-1] + word[i + 1:]
```
