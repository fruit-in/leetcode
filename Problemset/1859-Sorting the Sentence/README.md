# 1859. Sorting the Sentence
A **sentence** is a list of words that are separated by a single space with no leading or trailing spaces. Each word consists of lowercase and uppercase English letters.

A sentence can be **shuffled** by appending the **1-indexed word position** to each word then rearranging the words in the sentence.
* For example, the sentence `"This is a sentence"` can be shuffled as `"sentence4 a3 is2 This1"` or `"is2 sentence4 This1 a3"`.

Given a **shuffled sentence** `s` containing no more than `9` words, reconstruct and return *the original sentence*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "is2 sentence4 This1 a3"
<strong>Output:</strong> "This is a sentence"
<strong>Explanation:</strong> Sort the words in s to their original positions "This1 is2 a3 sentence4", then remove the numbers.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "Myself2 Me1 I4 and3"
<strong>Output:</strong> "Me Myself and I"
<strong>Explanation:</strong> Sort the words in s to their original positions "Me1 Myself2 and3 I4", then remove the numbers.
</pre>

#### Constraints:
* `2 <= s.length <= 200`
* `s` consists of lowercase and uppercase English letters, spaces, and digits from `1` to `9`.
* The number of words in `s` is between `1` and `9`.
* The words in `s` are separated by a single space.
* `s` contains no leading or trailing spaces.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def sortSentence(self, s: str) -> str:
        words = s.split()
        words.sort(key=lambda w: w[-1])

        return ' '.join(word[:-1] for word in words)
```
