# 745. Prefix and Suffix Search
Design a special dictionary that searches the words in it by a prefix and a suffix.

Implement the `WordFilter` class:
* `WordFilter(string[] words)` Initializes the object with the `words` in the dictionary.
* `f(string pref, string suff)` Returns *the index of the word in the dictionary*, which has the prefix `pref` and the suffix `suff`. If there is more than one valid index, return **the largest** of them. If there is no such word in the dictionary, return `-1`.

#### Example 1:
<pre>
<strong>Input:</strong>
["WordFilter", "f"]
[[["apple"]], ["a", "e"]]
<strong>Output:</strong>
[null, 0]
<strong>Explanation:</strong>
WordFilter wordFilter = new WordFilter(["apple"]);
wordFilter.f("a", "e"); // return 0, because the word at index 0 has prefix = "a" and suffix = "e".
</pre>

#### Constraints:
* <code>1 <= words.length <= 104</sup></code>
* `1 <= words[i].length <= 7`
* `1 <= pref.length, suff.length <= 7`
* `words[i]`, `pref` and `suff` consist of lowercase English letters only.
* At most <code>10<sup>4</sup></code> calls will be made to the function `f`.

## Solutions (Python)

### 1. Solution
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
