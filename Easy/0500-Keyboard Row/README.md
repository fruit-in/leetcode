# 500. Keyboard Row
Given a List of words, return the words that can be typed using letters of **alphabet** on only one row's of American keyboard like the image below.

![](https://assets.leetcode.com/uploads/2018/10/12/keyboard.png)

#### Example 1:
<pre>
<strong>Input:</strong> ["Hello", "Alaska", "Dad", "Peace"]
<strong>Output:</strong> ["Alaska", "Dad"]
</pre>

#### Note:
1. You may use one character in the keyboard more than once.
2. You may assume the input string will only contain letters of alphabet.

## Solutions

### 1. Set (Python3)
```Python3
class Solution:
    def findWords(self, words: List[str]) -> List[str]:
        result = []
        row_q = set("qwertyuiop")
        row_a = set("asdfghjkl")
        row_z = set("zxcvbnm")
        for word in words:
            word_set = set(word.lower())
            if word_set <= row_q or word_set <= row_a or word_set <= row_z:
                result.append(word)
        return result
```
