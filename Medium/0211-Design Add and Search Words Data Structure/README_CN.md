# 211. 添加与搜索单词 - 数据结构设计
如果数据结构中有任何与word匹配的字符串，则bool search（word）返回true，否则返回false。 单词可能包含点“.” 点可以与任何字母匹配的地方。

请你设计一个数据结构，支持 添加新单词 和 查找字符串是否与任何先前添加的字符串匹配 。

实现词典类 `WordDictionary` ：
* `WordDictionary()` 初始化词典对象
* `void addWord(word)` 将 `word` 添加到数据结构中，之后可以对它进行匹配
* `bool search(word)` 如果数据结构中存在字符串与 `word` 匹配，则返回 `true` ；否则，返回  `false` 。`word` 中可能包含一些 `'.'` ，每个 `.` 都可以表示任何一个字母。

#### 示例:
<pre>
<strong>输入:</strong>
["WordDictionary","addWord","addWord","addWord","search","search","search","search"]
[[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]
<strong>输出:</strong>
[null,null,null,null,false,true,true,true]
<strong>解释:</strong>
WordDictionary wordDictionary = new WordDictionary();
wordDictionary.addWord("bad");
wordDictionary.addWord("dad");
wordDictionary.addWord("mad");
wordDictionary.search("pad"); // return False
wordDictionary.search("bad"); // return True
wordDictionary.search(".ad"); // return True
wordDictionary.search("b.."); // return True
</pre>

#### 提示:
* `1 <= word.length <= 500`
* `addWord` 中的 `word` 由小写英文字母组成
* `search` 中的 `word` 由 '.' 或小写英文字母组成
* 最调用多 `50000` 次 `addWord` 和 `search`

## 题解 (Python)

### 1. 题解
```Python
class WordDictionary:

    def __init__(self):
        """
        Initialize your data structure here.
        """
        self.children = [None] * 26
        self.is_end = False

    def addWord(self, word: str) -> None:
        """
        Adds a word into the data structure.
        """
        i = ord(word[0]) - 97

        if not self.children[i]:
            self.children[i] = WordDictionary()

        if len(word) == 1:
            self.children[i].is_end = True
        else:
            self.children[i].addWord(word[1:])

    def search(self, word: str) -> bool:
        """
        Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter.
        """
        i = ord(word[0]) - 97

        if word == '.':
            return any(child.is_end for child in self.children if child)
        elif len(word) == 1:
            return self.children[i] and self.children[i].is_end
        elif word[0] == '.':
            return any(child.search(word[1:]) for child in self.children if child)
        else:
            return self.children[i] and self.children[i].search(word[1:])


# Your WordDictionary object will be instantiated and called as such:
# obj = WordDictionary()
# obj.addWord(word)
# param_2 = obj.search(word)
```
