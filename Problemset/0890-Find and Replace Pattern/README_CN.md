# 890. 查找和替换模式
你有一个单词列表 `words` 和一个模式  `pattern`，你想知道 `words` 中的哪些单词与模式匹配。

如果存在字母的排列 `p` ，使得将模式中的每个字母 `x` 替换为 `p(x)` 之后，我们就得到了所需的单词，那么单词与模式是匹配的。

*（回想一下，字母的排列是从字母到字母的双射：每个字母映射到另一个字母，没有两个字母映射到同一个字母。）*

返回 `words` 中与给定模式匹配的单词列表。

你可以按任何顺序返回答案。

#### 示例:
<pre>
<strong>输入:</strong> words = ["abc","deq","mee","aqq","dkd","ccc"], pattern = "abb"
<strong>输出:</strong> ["mee","aqq"]
<strong>解释:</strong>
"mee" 与模式匹配，因为存在排列 {a -> m, b -> e, ...}。
"ccc" 与模式不匹配，因为 {a -> c, b -> c, ...} 不是排列。
因为 a 和 b 映射到同一个字母。
</pre>

#### 提示:
* `1 <= words.length <= 50`
* `1 <= pattern.length = words[i].length <= 20`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def findAndReplacePattern(self, words: List[str], pattern: str) -> List[str]:
        ret = []

        for word in words:
            map_wp = {}
            map_pw = {}

            for wl, pl in zip(word, pattern):
                if wl not in map_wp and pl not in map_pw:
                    map_wp[wl] = pl
                    map_pw[pl] = wl
                elif wl not in map_wp or pl not in map_pw:
                    break
                elif map_wp[wl] != pl or map_pw[pl] != wl:
                    break
            else:
                ret.append(word)

        return ret
```
