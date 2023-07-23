# 211. Design Add and Search Words Data Structure
Design a data structure that supports adding new words and finding if a string matches any previously added string.

Implement the `WordDictionary` class:
* `WordDictionary()` Initializes the object.
* `void addWord(word)` Adds `word` to the data structure, it can be matched later.
* `bool search(word)` Returns `true` if there is any string in the data structure that matches `word` or `false` otherwise. `word` may contain dots `'.'` where dots can be matched with any letter.

#### Example:
<pre>
<strong>Input:</strong>
["WordDictionary","addWord","addWord","addWord","search","search","search","search"]
[[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]
<strong>Output:</strong>
[null,null,null,null,false,true,true,true]
<strong>Explanation:</strong>
WordDictionary wordDictionary = new WordDictionary();
wordDictionary.addWord("bad");
wordDictionary.addWord("dad");
wordDictionary.addWord("mad");
wordDictionary.search("pad"); // return False
wordDictionary.search("bad"); // return True
wordDictionary.search(".ad"); // return True
wordDictionary.search("b.."); // return True
</pre>

#### Constraints:
* `1 <= word.length <= 500`
* `word` in `addWord` consists lower-case English letters.
* `word` in `search` consist of  `'.'` or lower-case English letters.
* At most `50000` calls will be made to `addWord` and `search`.

## Solutions (Python)

### 1. Solution
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
