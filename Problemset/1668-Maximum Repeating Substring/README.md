# 1668. Maximum Repeating Substring
For a string `sequence`, a string `word` is **`k`-repeating** if `word` concatenated `k` times is a substring of `sequence`. The `word`'s **maximum `k`-repeating value** is the highest value `k` where `word` is `k`-repeating in `sequence`. If `word` is not a substring of `sequence`, `word`'s maximum `k`-repeating value is `0`.

Given strings `sequence` and `word`, return *the **maximum `k`-repeating value** of `word` in `sequence`*.

#### Example 1:
<pre>
<strong>Input:</strong> sequence = "ababc", word = "ab"
<strong>Output:</strong> 2
<strong>Explanation:</strong> "abab" is a substring in "<u>abab</u>c".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> sequence = "ababc", word = "ba"
<strong>Output:</strong> 1
<strong>Explanation:</strong> "ba" is a substring in "a<u>ba</u>bc". "baba" is not a substring in "ababc".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> sequence = "ababc", word = "ac"
<strong>Output:</strong> 0
<strong>Explanation:</strong> "ac" is not a substring in "ababc".
</pre>

#### Constraints:
* `1 <= sequence.length <= 100`
* `1 <= word.length <= 100`
* `sequence` and `word` contains only lowercase English letters.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def maxRepeating(self, sequence: str, word: str) -> int:
        i = 0
        k = 0

        while i + (k + 1) * len(word) <= len(sequence):
            if sequence[i:i + (k + 1) * len(word)] == (k + 1) * word:
                k += 1
            else:
                i += 1

        return k
```

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {String} sequence
# @param {String} word
# @return {Integer}
def max_repeating(sequence, word)
  i = 0
  k = 0

  while i + (k + 1) * word.length <= sequence.length
    if sequence[i...i + (k + 1) * word.length] == word * (k + 1)
      k += 1
    else
      i += 1
    end
  end

  k
end
```
