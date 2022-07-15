# 2062. Count Vowel Substrings of a String
A **substring** is a contiguous (non-empty) sequence of characters within a string.

A **vowel substring** is a substring that **only** consists of vowels (`'a'`, `'e'`, `'i'`, `'o'`, and `'u'`) and has **all five** vowels present in it.

Given a string `word`, return *the number of **vowel substrings** in* `word`.

#### Example 1:
<pre>
<strong>Input:</strong> word = "aeiouu"
<strong>Output:</strong> 2
<strong>Explanation:</strong> The vowel substrings of word are as follows (underlined):
- "aeiouu"
- "aeiouu"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> word = "unicornarihan"
<strong>Output:</strong> 0
<strong>Explanation:</strong> Not all 5 vowels are present, so there are no vowel substrings.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> word = "cuaieuouac"
<strong>Output:</strong> 7
<strong>Explanation:</strong> The vowel substrings of word are as follows (underlined):
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"
</pre>

#### Constraints:
* `1 <= word.length <= 100`
* `word` consists of lowercase English letters only.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def countVowelSubstrings(self, word: str) -> int:
        ret = 0

        for i in range(len(word)):
            aeiou = [False] * 5
            for j in range(i, len(word)):
                if word[j] not in "aeiou":
                    break

                aeiou[0] |= word[j] == 'a'
                aeiou[1] |= word[j] == 'e'
                aeiou[2] |= word[j] == 'i'
                aeiou[3] |= word[j] == 'o'
                aeiou[4] |= word[j] == 'u'

                if all(aeiou):
                    ret += 1

        return ret
```
