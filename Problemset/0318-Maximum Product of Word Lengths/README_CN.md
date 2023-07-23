# 318. 最大单词长度乘积
给定一个字符串数组 ```words```，找到 ```length(word[i]) * length(word[j])``` 的最大值，并且这两个单词不含有公共字母。你可以认为每个单词只包含小写字母。如果不存在这样的两个单词，返回 0。

#### 示例 1:
<pre>
<strong>输入:</strong> ["abcw","baz","foo","bar","xtfn","abcdef"]
<strong>输出:</strong> 16
<strong>解释:</strong> 这两个单词为 "abcw", "xtfn"。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> ["a","ab","abc","d","cd","bcd","abcd"]
<strong>输出:</strong> 4
<strong>解释:</strong> 这两个单词为 "ab", "cd"。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> ["a","aa","aaa","aaaa"]
<strong>输出:</strong> 0
<strong>解释:</strong> 不存在这样的两个单词。
</pre>

## 题解 (Python)

### 1. 题解
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
