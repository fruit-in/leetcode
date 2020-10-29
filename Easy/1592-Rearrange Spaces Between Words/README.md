# 1592. Rearrange Spaces Between Words
You are given a string `text` of words that are placed among some number of spaces. Each word consists of one or more lowercase English letters and are separated by at least one space. It's guaranteed that `text` **contains at least one word**.

Rearrange the spaces so that there is an **equal** number of spaces between every pair of adjacent words and that number is **maximized**. If you cannot redistribute all the spaces equally, place the **extra spaces at the end**, meaning the returned string should be the same length as `text`.

Return *the string after rearranging the spaces*.

#### Example 1:
<pre>
<b>Input:</b> text = "  this   is  a sentence "
<b>Output:</b> "this   is   a   sentence"
<b>Explanation:</b> There are a total of 9 spaces and 4 words. We can evenly divide the 9 spaces between the words: 9 / (4-1) = 3 spaces.
</pre>

#### Example 2:
<pre>
<b>Input:</b> text = " practice   makes   perfect"
<b>Output:</b> "practice   makes   perfect "
<b>Explanation:</b> There are a total of 7 spaces and 3 words. 7 / (3-1) = 3 spaces plus 1 extra space. We place this extra space at the end of the string.
</pre>

#### Example 3:
<pre>
<b>Input:</b> text = "hello   world"
<b>Output:</b> "hello   world"
</pre>

#### Example 4:
<pre>
<b>Input:</b> text = "  walks  udp package   into  bar a"
<b>Output:</b> "walks  udp  package  into  bar  a "
</pre>

#### Example 5:
<pre>
<b>Input:</b> text = "a"
<b>Output:</b> "a"
</pre>

#### Constraints:
* `1 <= text.length <= 100`
* `text` consists of lowercase English letters and `' '`.
* `text` contains at least one word.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def reorderSpaces(self, text: str) -> str:
        words = list(filter(lambda x: x != '', text.split(' ')))
        if len(words) == 1:
            div, mod = 0, text.count(' ')
        else:
            div, mod = divmod(text.count(' '), len(words) - 1)

        return (div * ' ').join(words) + mod * ' '
```
