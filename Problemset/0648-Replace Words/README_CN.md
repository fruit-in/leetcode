# 648. 单词替换
在英语中，我们有一个叫做 `词根`(root) 的概念，可以词根**后面**添加其他一些词组成另一个较长的单词——我们称这个词为 `继承词`(successor)。例如，词根`an`，跟随着单词 `other`(其他)，可以形成新的单词 `another`(另一个)。

现在，给定一个由许多**词根**组成的词典 `dictionary` 和一个用空格分隔单词形成的句子 `sentence`。你需要将句子中的所有**继承词**用**词根**替换掉。如果**继承词**有许多可以形成它的**词根**，则用**最短**的词根替换它。

你需要输出替换之后的句子。

#### 示例 1:
<pre>
<strong>输入:</strong> dictionary = ["cat","bat","rat"], sentence = "the cattle was rattled by the battery"
<strong>输出:</strong> "the cat was rat by the bat"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> dictionary = ["a","b","c"], sentence = "aadsfasf absbs bbab cadsfafs"
<strong>输出:</strong> "a a b c"
</pre>

#### 提示:
* `1 <= dictionary.length <= 1000`
* `1 <= dictionary[i].length <= 100`
* `dictionary[i]` 仅由小写字母组成。
* <code>1 <= sentence.length <= 10<sup>6</sup></code>
* `sentence` 仅由小写字母和空格组成。
* `sentence` 中单词的总量在范围 `[1, 1000]` 内。
* `sentence` 中每个单词的长度在范围 `[1, 1000]` 内。
* `sentence` 中单词之间由一个空格隔开。
* `sentence` 没有前导或尾随空格。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def replaceWords(self, dictionary: List[str], sentence: str) -> str:
        trie = {}
        words = sentence.split()

        for root in dictionary:
            curr = trie

            for i in range(len(root)):
                if root[i] not in curr:
                    curr[root[i]] = {}
                curr = curr[root[i]]
                if i == len(root) - 1:
                    curr[''] = {}

        for i in range(len(words)):
            curr = trie

            for j in range(len(words[i])):
                if words[i][j] not in curr:
                    break
                curr = curr[words[i][j]]
                if '' in curr:
                    words[i] = words[i][:j + 1]
                    break

        return ' '.join(words)
```
