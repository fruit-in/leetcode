# 884. 两句话中的不常见单词
给定两个句子 ```A``` 和 ```B``` 。 （*句子*是一串由空格分隔的单词。每个*单词*仅由小写字母组成。）

如果一个单词在其中一个句子中只出现一次，在另一个句子中却没有出现，那么这个单词就是*不常见的*。

返回所有不常用单词的列表。

您可以按任何顺序返回列表。

#### 示例 1:
<pre>
<strong>输入:</strong> A = "this apple is sweet", B = "this apple is sour"
<strong>输出:</strong> ["sweet","sour"]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> A = "apple apple", B = "banana"
<strong>输出:</strong> ["banana"]
</pre>

#### 提示:
1. ```0 <= A.length <= 200```
2. ```0 <= B.length <= 200```
3. ```A``` 和 ```B``` 都只包含空格和小写字母。

## 题解 (Python)

### 1. 计数
```Python
class Solution:
    def uncommonFromSentences(self, A: str, B: str) -> List[str]:
        word_count = {}
        ret = []

        for word in (A + ' ' + B).split(' '):
            if word_count.get(word):
                word_count[word] += 1
            else:
                word_count[word] = 1

        for word, count in word_count.items():
            if count == 1:
                ret.append(word)

        return ret
```
