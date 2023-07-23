# 2062. 统计字符串中的元音子字符串
**子字符串** 是字符串中的一个连续（非空）的字符序列。

**元音子字符串** 是 **仅** 由元音（`'a'`、`'e'`、`'i'`、`'o'` 和 `'u'`）组成的一个子字符串，且必须包含 **全部五种** 元音。

给你一个字符串 `word` ，统计并返回 `word` 中 **元音子字符串的数目** 。

#### 示例 1:
<pre>
<strong>输入:</strong> word = "aeiouu"
<strong>输出:</strong> 2
<strong>解释:</strong> 下面列出 word 中的元音子字符串（斜体加粗部分）：
- "aeiouu"
- "aeiouu"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> word = "unicornarihan"
<strong>输出:</strong> 0
<strong>解释:</strong> word 中不含 5 种元音，所以也不会存在元音子字符串。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> word = "cuaieuouac"
<strong>输出:</strong> 7
<strong>解释:</strong> 下面列出 word 中的元音子字符串（斜体加粗部分）：
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> word = "bbaeixoubb"
<strong>输出:</strong> 0
<strong>解释:</strong> 所有包含全部五种元音的子字符串都含有辅音，所以不存在元音子字符串。
</pre>

#### 提示:
* `1 <= word.length <= 100`
* `word` 仅由小写英文字母组成

## 题解 (Python)

### 1. 题解
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
