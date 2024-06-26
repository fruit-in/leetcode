# 1048. Longest String Chain
You are given an array of `words` where each word consists of lowercase English letters.

<code>word<sub>A</sub></code> is a **predecessor** of <code>word<sub>B</sub></code> if and only if we can insert **exactly one** letter anywhere in <code>word<sub>A</sub></code> **without changing the order of the other characters** to make it equal to <code>word<sub>B</sub></code>.

* For example, `"abc"` is a **predecessor** of `"abac"`, while `"cba"` is not a **predecessor** of `"bcad"`.

A **word chain** is a sequence of words <code>[word<sub>1</sub>, word<sub>2</sub>, ..., word<sub>k</sub>]</code> with `k >= 1`, where <code>word<sub>1</sub></code> is a **predecessor** of <code>word<sub>2</sub></code>, <code>word<sub>2</sub></code> is a **predecessor** of <code>word<sub>3</sub></code>, and so on. A single word is trivially a **word chain** with `k == 1`.

Return *the **length** of the **longest possible word chain** with words chosen from the given list of* `words`.

#### Example 1:
<pre>
<strong>Input:</strong> words = ["a","b","ba","bca","bda","bdca"]
<strong>Output:</strong> 4
<strong>Explanation:</strong> One of the longest word chains is ["a","ba","bda","bdca"].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words = ["xbc","pcxbcf","xb","cxbc","pcxbc"]
<strong>Output:</strong> 5
<strong>Explanation:</strong> All the words can be put in a word chain ["xb", "xbc", "cxbc", "pcxbc", "pcxbcf"].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> words = ["abcd","dbqca"]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The trivial word chain ["abcd"] is one of the longest word chains.
["abcd","dbqca"] is not a valid word chain because the ordering of the letters is changed.
</pre>

#### Constraints:
* `1 <= words.length <= 1000`
* `1 <= words[i].length <= 16`
* `words[i]` only consists of lowercase English letters.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def longestStrChain(self, words: List[str]) -> int:
        chainlen = {word: 1 for word in words}

        for word in sorted(words, key=len):
            for i in range(len(word)):
                pred = word[:i] + word[i + 1:]
                chainlen[word] = max(chainlen[word], chainlen.get(pred, 0) + 1)

        return max(chainlen.values())
```
