# 745. 前缀和后缀搜索
设计一个包含一些单词的特殊词典，并能够通过前缀和后缀来检索单词。

实现 `WordFilter` 类：
* `WordFilter(string[] words)` 使用词典中的单词 `words` 初始化对象。
* `f(string pref, string suff)` 返回词典中具有前缀 `pref` 和后缀 `suff` 的单词的下标。如果存在不止一个满足要求的下标，返回其中 **最大的下标** 。如果不存在这样的单词，返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong>
["WordFilter", "f"]
[[["apple"]], ["a", "e"]]
<strong>输出:</strong>
[null, 0]
<strong>解释:</strong>
WordFilter wordFilter = new WordFilter(["apple"]);
wordFilter.f("a", "e"); // 返回 0 ，因为下标为 0 的单词：前缀 prefix = "a" 且 后缀 suffix = "e" 。
</pre>

#### 提示:
* <code>1 <= words.length <= 104</sup></code>
* `1 <= words[i].length <= 7`
* `1 <= pref.length, suff.length <= 7`
* `words[i]`、`pref` 和 `suff` 仅由小写英文字母组成
* 最多对函数 `f` 执行 <code>10<sup>4</sup></code> 次调用

## 题解 (Python)

### 1. 题解
```Python
class WordFilter:

    def __init__(self, words: List[str]):
        self.hashmap = {}

        for i in range(len(words)):
            prefs = [words[i][:j + 1] for j in range(len(words[i]))]
            suffs = [words[i][-j - 1:] for j in range(len(words[i]))]

            for pref in prefs:
                for suff in suffs:
                    self.hashmap[(pref, suff)] = i

    def f(self, pref: str, suff: str) -> int:
        return self.hashmap.get((pref, suff), -1)


# Your WordFilter object will be instantiated and called as such:
# obj = WordFilter(words)
# param_1 = obj.f(pref,suff)
```
