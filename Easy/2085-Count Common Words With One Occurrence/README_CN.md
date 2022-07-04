# 2085. 统计出现过一次的公共字符串
给你两个字符串数组 `words1` 和 `words2` ，请你返回在两个字符串数组中 **都恰好出现一次** 的字符串的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> words1 = ["leetcode","is","amazing","as","is"], words2 = ["amazing","leetcode","is"]
<strong>输出:</strong> 2
<strong>解释:</strong>
- "leetcode" 在两个数组中都恰好出现一次，计入答案。
- "amazing" 在两个数组中都恰好出现一次，计入答案。
- "is" 在两个数组中都出现过，但在 words1 中出现了 2 次，不计入答案。
- "as" 在 words1 中出现了一次，但是在 words2 中没有出现过，不计入答案。
所以，有 2 个字符串在两个数组中都恰好出现了一次。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words1 = ["b","bb","bbb"], words2 = ["a","aa","aaa"]
<strong>输出:</strong> 0
<strong>解释:</strong> 没有字符串在两个数组中都恰好出现一次。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> words1 = ["a","ab"], words2 = ["a","a","a","ab"]
<strong>输出:</strong> 1
<strong>解释:</strong> 唯一在两个数组中都出现一次的字符串是 "ab" 。
</pre>

#### 提示:
* `1 <= words1.length, words2.length <= 1000`
* `1 <= words1[i].length, words2[j].length <= 30`
* `words1[i]` 和 `words2[j]` 都只包含小写英文字母。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def countWords(self, words1: List[str], words2: List[str]) -> int:
        count1 = Counter(words1)
        count2 = Counter(words2)

        return sum(1 for k, v in count1.items() if v == 1 and count2[k] == 1)
```
