# 2490. Circular Sentence
A **sentence** is a list of words that are separated by a **single** space with no leading or trailing spaces.
* For example, `"Hello World"`, `"HELLO"`, `"hello world hello world"` are all sentences.

Words consist of **only** uppercase and lowercase English letters. Uppercase and lowercase English letters are considered different.

A sentence is **circular** if:
* The last character of a word is equal to the first character of the next word.
* The last character of the last word is equal to the first character of the first word.

For example, `"leetcode exercises sound delightful"`, `"eetcode"`, `"leetcode eats soul"` are all circular sentences. However, `"Leetcode is cool"`, `"happy Leetcode"`, `"Leetcode"` and `"I like Leetcode"` are **not** circular sentences.

Given a string `sentence`, return `true` *if it is circular*. Otherwise, return `false`.

#### Example 1:
<pre>
<strong>Input:</strong> sentence = "leetcode exercises sound delightful"
<strong>Output:</strong> true
<strong>Explanation:</strong> The words in sentence are ["leetcode", "exercises", "sound", "delightful"].
- leetcode's last character is equal to exercises's first character.
- exercises's last character is equal to sound's first character.
- sound's last character is equal to delightful's first character.
- delightful's last character is equal to leetcode's first character.
The sentence is circular.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> sentence = "eetcode"
<strong>Output:</strong> true
<strong>Explanation:</strong> The words in sentence are ["eetcode"].
- eetcode's last character is equal to eetcode's first character.
The sentence is circular.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> sentence = "Leetcode is cool"
<strong>Output:</strong> false
<strong>Explanation:</strong> The words in sentence are ["Leetcode", "is", "cool"].
- Leetcode's last character is not equal to is's first character.
The sentence is not circular.
</pre>

#### Constraints:
* `1 <= sentence.length <= 500`
* `sentence` consist of only lowercase and uppercase English letters and spaces.
* The words in `sentence` are separated by a single space.
* There are no leading or trailing spaces.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def isCircularSentence(self, sentence: str) -> bool:
        for i in range(len(sentence)):
            if sentence[i] == ' ' and sentence[i - 1] != sentence[i + 1]:
                return False

        return sentence[-1] == sentence[0]
```
