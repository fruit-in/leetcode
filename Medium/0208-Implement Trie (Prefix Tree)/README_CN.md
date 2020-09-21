# 208. 实现 Trie (前缀树)
实现一个 Trie (前缀树)，包含 `insert`, `search`, 和 `startsWith` 这三个操作。

#### 示例:
```
Trie trie = new Trie();

trie.insert("apple");
trie.search("apple");   // 返回 true
trie.search("app");     // 返回 false
trie.startsWith("app"); // 返回 true
trie.insert("app");
trie.search("app");     // 返回 true
```

#### 说明:
* 你可以假设所有的输入都是由小写字母 `a-z` 构成的。
* 保证所有输入均为非空字符串。

## 题解 (Python)

### 1. 递归
```Python
class Trie:

    def __init__(self, x=''):
        """
        Initialize your data structure here.
        """
        self.val = x
        self.children = []

    def insert(self, word: str) -> None:
        """
        Inserts a word into the trie.
        """
        if not word and '' not in [child.val for child in self.children]:
            self.children.append(Trie())
        elif word:
            for child in self.children:
                if child.val == word[0]:
                    child.insert(word[1:])
                    return None

            self.children.append(Trie(word[0]))
            self.children[-1].insert(word[1:])

    def search(self, word: str) -> bool:
        """
        Returns if the word is in the trie.
        """
        if not word:
            return '' in [child.val for child in self.children]

        for child in self.children:
            if child.val == word[0]:
                return child.search(word[1:])

        return False

    def startsWith(self, prefix: str) -> bool:
        """
        Returns if there is any word in the trie that starts with the given prefix.
        """
        if not prefix:
            return True

        for child in self.children:
            if child.val == prefix[0]:
                return child.startsWith(prefix[1:])

        return False


# Your Trie object will be instantiated and called as such:
# obj = Trie()
# obj.insert(word)
# param_2 = obj.search(word)
# param_3 = obj.startsWith(prefix)
```
