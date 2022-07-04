# 2129. Capitalize the Title
You are given a string `title` consisting of one or more words separated by a single space, where each word consists of English letters. **Capitalize** the string by changing the capitalization of each word such that:
* If the length of the word is `1` or `2` letters, change all letters to lowercase.
* Otherwise, change the first letter to uppercase and the remaining letters to lowercase.

Return *the **capitalized*** `title`.

#### Example 1:
<pre>
<strong>Input:</strong> title = "capiTalIze tHe titLe"
<strong>Output:</strong> "Capitalize The Title"
<strong>Explanation:</strong>
Since all the words have a length of at least 3, the first letter of each word is uppercase, and the remaining letters are lowercase.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> title = "First leTTeR of EACH Word"
<strong>Output:</strong> "First Letter of Each Word"
<strong>Explanation:</strong>
The word "of" has length 2, so it is all lowercase.
The remaining words have a length of at least 3, so the first letter of each remaining word is uppercase, and the remaining letters are lowercase.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> title = "i lOve leetcode"
<strong>Output:</strong> "i Love Leetcode"
<strong>Explanation:</strong>
The word "i" has length 1, so it is lowercase.
The remaining words have a length of at least 3, so the first letter of each remaining word is uppercase, and the remaining letters are lowercase.
</pre>

#### Constraints:
* `1 <= title.length <= 100`
* `title` consists of words separated by a single space without any leading or trailing spaces.
* Each word consists of uppercase and lowercase English letters and is **non-empty**.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def capitalizeTitle(self, title: str) -> str:
        return ' '.join(w.lower() if len(w) < 3 else w.capitalize() for w in title.split())
```
