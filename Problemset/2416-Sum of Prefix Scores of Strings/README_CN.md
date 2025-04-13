# 2416. 字符串的前缀分数和
给你一个长度为 `n` 的数组 `words` ，该数组由 **非空** 字符串组成。

定义字符串 `term` 的 **分数** 等于以 `term` 作为 **前缀** 的 `words[i]` 的数目。

* 例如，如果 `words = ["a", "ab", "abc", "cab"]` ，那么 `"ab"` 的分数是 `2` ，因为 `"ab"` 是 `"ab"` 和 `"abc"` 的一个前缀。

返回一个长度为 `n` 的数组 `answer` ，其中 `answer[i]` 是 `words[i]` 的每个非空前缀的分数 **总和** 。

**注意：**字符串视作它自身的一个前缀。

#### 示例 1:
<pre>
<strong>输入:</strong> words = ["abc","ab","bc","b"]
<strong>输出:</strong> [5,4,3,2]
<strong>解释:</strong> 对应每个字符串的答案如下：
- "abc" 有 3 个前缀："a"、"ab" 和 "abc" 。
- 2 个字符串的前缀为 "a" ，2 个字符串的前缀为 "ab" ，1 个字符串的前缀为 "abc" 。
总计 answer[0] = 2 + 2 + 1 = 5 。
- "ab" 有 2 个前缀："a" 和 "ab" 。
- 2 个字符串的前缀为 "a" ，2 个字符串的前缀为 "ab" 。
总计 answer[1] = 2 + 2 = 4 。
- "bc" 有 2 个前缀："b" 和 "bc" 。
- 2 个字符串的前缀为 "b" ，1 个字符串的前缀为 "bc" 。
总计 answer[2] = 2 + 1 = 3 。
- "b" 有 1 个前缀："b"。
- 2 个字符串的前缀为 "b" 。
总计 answer[3] = 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words = ["abcd"]
<strong>输出:</strong> [4]
<strong>解释:</strong>
"abcd" 有 4 个前缀 "a"、"ab"、"abc" 和 "abcd"。
每个前缀的分数都是 1 ，总计 answer[0] = 1 + 1 + 1 + 1 = 4 。
</pre>

#### 提示:
* `1 <= words.length <= 1000`
* `1 <= words[i].length <= 1000`
* `words[i]` 由小写英文字母组成

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def sumPrefixScores(self, words: List[str]) -> List[int]:
        root = {}
        answer = [0] * len(words)

        for word in words:
            curr = root

            for c in word:
                if c not in curr:
                    curr[c] = [0, {}]
                curr[c][0] += 1
                curr = curr[c][1]

        for i, word in enumerate(words):
            curr = root

            for c in word:
                answer[i] += curr[c][0]
                curr = curr[c][1]

        return answer
```
