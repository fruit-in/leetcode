# 676. Implement Magic Dictionary
Design a data structure that is initialized with a list of **different** words. Provided a string, you should determine if you can change exactly one character in this string to match any word in the data structure.

Implement the `MagicDictionary` class:

* `MagicDictionary()` Initializes the object.
* `void buildDict(String[] dictionary)` Sets the data structure with an array of distinct strings `dictionary`.
* `bool search(String searchWord)` Returns `true` if you can change **exactly one character** in `searchWord` to match any string in the data structure, otherwise returns `false`.

#### Example 1:
<pre>
<strong>Input:</strong>
["MagicDictionary", "buildDict", "search", "search", "search", "search"]
[[], [["hello", "leetcode"]], ["hello"], ["hhllo"], ["hell"], ["leetcoded"]]
<strong>Output:</strong>
[null, null, false, true, false, false]
<strong>Explanation:</strong>
MagicDictionary magicDictionary = new MagicDictionary();
magicDictionary.buildDict(["hello", "leetcode"]);
magicDictionary.search("hello"); // return False
magicDictionary.search("hhllo"); // We can change the second 'h' to 'e' to match "hello" so we return True
magicDictionary.search("hell"); // return False
magicDictionary.search("leetcoded"); // return False
</pre>

#### Constraints:
* `1 <= dictionary.length <= 100`
* `1 <= dictionary[i].length <= 100`
* `dictionary[i]` consists of only lower-case English letters.
* All the strings in `dictionary` are **distinct**.
* `1 <= searchWord.length <= 100`
* `searchWord` consists of only lower-case English letters.
* `buildDict` will be called only once before `search`.
* At most `100` calls will be made to `search`.

## Solutions (Python)

### 1. Solution
```Python
class MagicDictionary:

    def __init__(self):
        self.presufban = {}

    def buildDict(self, dictionary: List[str]) -> None:
        for word in dictionary:
            for i in range(len(word)):
                prefix = word[:i]
                suffix = word[i + 1:]

                if (prefix, suffix) not in self.presufban:
                    self.presufban[(prefix, suffix)] = word[i]
                else:
                    self.presufban[(prefix, suffix)] = '?'

    def search(self, searchWord: str) -> bool:
        for i in range(len(searchWord)):
            prefix = searchWord[:i]
            suffix = searchWord[i + 1:]

            if self.presufban.get((prefix, suffix), searchWord[i]) != searchWord[i]:
                return True

        return False


# Your MagicDictionary object will be instantiated and called as such:
# obj = MagicDictionary()
# obj.buildDict(dictionary)
# param_2 = obj.search(searchWord)
```
