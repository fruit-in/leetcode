# 2451. Odd String Difference
You are given an array of equal-length strings `words`. Assume that the length of each string is `n`.

Each string `words[i]` can be converted into a **difference integer array** `difference[i]` of length `n - 1` where `difference[i][j] = words[i][j+1] - words[i][j]` where `0 <= j <= n - 2`. Note that the difference between two letters is the difference between their **positions** in the alphabet i.e. the position of `'a'` is `0`, `'b'` is `1`, and `'z'` is `25`.

* For example, for the string `"acb"`, the difference integer array is `[2 - 0, 1 - 2] = [2, -1]`.

All the strings in words have the same difference integer array, **except one**. You should find that string.

Return *the string in* `words` *that has different **difference integer array***.

#### Example 1:
<pre>
<strong>Input:</strong> words = ["adc","wzy","abc"]
<strong>Output:</strong> "abc"
<strong>Explanation:</strong>
- The difference integer array of "adc" is [3 - 0, 2 - 3] = [3, -1].
- The difference integer array of "wzy" is [25 - 22, 24 - 25]= [3, -1].
- The difference integer array of "abc" is [1 - 0, 2 - 1] = [1, 1].
The odd array out is [1, 1], so we return the corresponding string, "abc".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words = ["aaa","bob","ccc","ddd"]
<strong>Output:</strong> "bob"
<strong>Explanation:</strong> All the integer arrays are [0, 0] except for "bob", which corresponds to [13, -13].
</pre>

#### Constraints:
* `3 <= words.length <= 100`
* `n == words[i].length`
* `2 <= n <= 20`
* `words[i]` consists of lowercase English letters.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def oddString(self, words: List[str]) -> str:
        n = len(words[0])
        difference = []

        for i in range(len(words)):
            difference.append([])
            for j in range(n - 1):
                difference[i].append(ord(words[i][j + 1]) - ord(words[i][j]))

            if i >= 2:
                prev = difference[i - 1] == difference[i - 2]
                curr = difference[i] == difference[i - 1]
                if prev and not curr:
                    return words[i]
                elif not prev and curr:
                    return words[i - 2]
                elif not prev and not curr:
                    return words[i - 1]
```
