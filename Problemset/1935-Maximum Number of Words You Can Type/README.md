# 1935. Maximum Number of Words You Can Type
There is a malfunctioning keyboard where some letter keys do not work. All other keys on the keyboard work properly.

Given a string `text` of words separated by a single space (no leading or trailing spaces) and a string `brokenLetters` of all **distinct** letter keys that are broken, return *the **number of words** in* `text` *you can fully type using this keyboard*.

#### Example 1:
<pre>
<strong>Input:</strong> text = "hello world", brokenLetters = "ad"
<strong>Output:</strong> 1
<strong>Explanation:</strong> We cannot type "world" because the 'd' key is broken.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> text = "leet code", brokenLetters = "lt"
<strong>Output:</strong> 1
<strong>Explanation:</strong> We cannot type "leet" because the 'l' and 't' keys are broken.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> text = "leet code", brokenLetters = "e"
<strong>Output:</strong> 0
<strong>Explanation:</strong> We cannot type either word because the 'e' key is broken.
</pre>

#### Constraints:
* <code>1 <= text.length <= 10<sup>4</sup></code>
* `0 <= brokenLetters.length <= 26`
* `text` consists of words separated by a single space without any leading or trailing spaces.
* Each word only consists of lowercase English letters.
* `brokenLetters` consists of **distinct** lowercase English letters.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def canBeTypedWords(self, text: str, brokenLetters: str) -> int:
        ret = 0

        for word in text.split():
            if all(c not in brokenLetters for c in word):
                ret += 1

        return ret
```
