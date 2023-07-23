# 1816. Truncate Sentence
A **sentence** is a list of words that are separated by a single space with no leading or trailing spaces. Each of the words consists of **only** uppercase and lowercase English letters (no punctuation).

* For example, `"Hello World"`, `"HELLO"`, and `"hello world hello world"` are all sentences.

You are given a sentence `s` and an integer `k`. You want to **truncate** `s` such that it contains only the **first** `k` words. Return `s` *after **truncating** it*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "Hello how are you Contestant", k = 4
<strong>Output:</strong> "Hello how are you"
<strong>Explanation:</strong>
The words in s are ["Hello", "how" "are", "you", "Contestant"].
The first 4 words are ["Hello", "how", "are", "you"].
Hence, you should return "Hello how are you".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "What is the solution to this problem", k = 4
<strong>Output:</strong> "What is the solution"
<strong>Explanation:</strong>
The words in s are ["What", "is" "the", "solution", "to", "this", "problem"].
The first 4 words are ["What", "is", "the", "solution"].
Hence, you should return "What is the solution".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "chopper is not a tanuki", k = 5
<strong>Output:</strong> "chopper is not a tanuki"
</pre>

#### Constraints:
* `1 <= s.length <= 500`
* `k` is in the range `[1, the number of words in s]`.
* `s` consist of only lowercase and uppercase English letters and spaces.
* The words in `s` are separated by a single space.
* There are no leading or trailing spaces.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def truncateSentence(self, s: str, k: int) -> str:
        return ' '.join(s.split()[:k])
```
