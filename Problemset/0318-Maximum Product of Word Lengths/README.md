# 318. Maximum Product of Word Lengths
Given a string array ```words```, find the maximum value of ```length(word[i]) * length(word[j])``` where the two words do not share common letters. You may assume that each word will contain only lower case letters. If no such two words exist, return 0.

#### Example 1:
<pre>
<strong>Input:</strong> ["abcw","baz","foo","bar","xtfn","abcdef"]
<strong>Output:</strong> 16
<strong>Explanation:</strong> The two words can be "abcw", "xtfn".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> ["a","ab","abc","d","cd","bcd","abcd"]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The two words can be "ab", "cd".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> ["a","aa","aaa","aaaa"]
<strong>Output:</strong> 0
<strong>Explanation:</strong> No such pair of words.
</pre>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def maxProduct(self, words: List[str]) -> int:
        mask_len = {}
        ret = 0

        for word in words:
            mask = 0
            for ch in word:
                mask |= 1 << (ord(ch) - 97)
            if mask not in mask_len:
                mask_len[mask] = 0
            mask_len[mask] = max(mask_len[mask], len(word))

        mask_len = list(mask_len.items())
        for i in range(len(mask_len)):
            for j in range(i + 1, len(mask_len)):
                if mask_len[i][0] & mask_len[j][0] == 0:
                    ret = max(ret, mask_len[i][1] * mask_len[j][1])

        return ret
```
