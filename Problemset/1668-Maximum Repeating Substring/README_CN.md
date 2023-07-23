# 1668. 最大重复子字符串
给你一个字符串 `sequence` ，如果字符串 `word` 连续重复 `k` 次形成的字符串是 `sequence` 的一个子字符串，那么单词 `word` 的 **重复值为 `k`** 。单词 `word` 的 **最大重复值** 是单词 `word` 在 `sequence` 中最大的重复值。如果 `word` 不是 `sequence` 的子串，那么重复值 `k` 为 `0` 。

给你一个字符串 `sequence` 和 `word` ，请你返回 **最大重复值 `k`** 。

#### 示例 1:
<pre>
<strong>输入:</strong> sequence = "ababc", word = "ab"
<strong>输出:</strong> 2
<strong>解释:</strong> "abab" 是 "<b>abab</b>c" 的子字符串。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> sequence = "ababc", word = "ba"
<strong>输出:</strong> 1
<strong>解释:</strong> "ba" 是 "a<b>ba</b>bc" 的子字符串，但 "baba" 不是 "ababc" 的子字符串。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> sequence = "ababc", word = "ac"
<strong>输出:</strong> 0
<strong>解释:</strong> "ac" 不是 "ababc" 的子字符串。
</pre>

#### 提示:
* `1 <= sequence.length <= 100`
* `1 <= word.length <= 100`
* `sequence` 和 `word` 都只包含小写英文字母。

## 题解 (Python)

### 1. 题解
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

## 题解 (Ruby)

### 1. 题解
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
