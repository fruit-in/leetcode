# 1048. 最长字符串链
给出一个单词数组 `words` ，其中每个单词都由小写英文字母组成。

如果我们可以 **不改变其他字符的顺序** ，在 <code>word<sub>A</sub></code> 的任何地方添加 **恰好一个** 字母使其变成 <code>word<sub>B</sub></code> ，那么我们认为 <code>word<sub>A</sub></code> 是 <code>word<sub>B</sub></code> 的 **前身** 。

* 例如，`"abc"` 是 `"abac"` 的 **前身** ，而 `"cba"` 不是 `"bcad"` 的 **前身**

**词链**是单词 `[word_1, word_2, ..., word_k]` 组成的序列，`k >= 1`，其中 <code>word<sub>1</sub></code> 是 <code>word<sub>2</sub></code> 的前身，<code>word<sub>2</sub></code> 是 <code>word<sub>3</sub></code> 的前身，依此类推。一个单词通常是 `k == 1` 的 **单词链** 。

从给定单词列表 `words` 中选择单词组成词链，返回 词链的 **最长可能长度** 。

#### 示例 1:
<pre>
<strong>输入:</strong> words = ["a","b","ba","bca","bda","bdca"]
<strong>输出:</strong> 4
<strong>解释:</strong> 最长单词链之一为 ["a","ba","bda","bdca"]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words = ["xbc","pcxbcf","xb","cxbc","pcxbc"]
<strong>输出:</strong> 5
<strong>解释:</strong> 所有的单词都可以放入单词链 ["xb", "xbc", "cxbc", "pcxbc", "pcxbcf"].
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> words = ["abcd","dbqca"]
<strong>输出:</strong> 1
<strong>解释:</strong> 字链["abcd"]是最长的字链之一。
["abcd"，"dbqca"]不是一个有效的单词链，因为字母的顺序被改变了。
</pre>

#### 提示:
* `1 <= words.length <= 1000`
* `1 <= words[i].length <= 16`
* `words[i]` 仅由小写英文字母组成。

## 题解 (Python)

### 1. 题解
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
