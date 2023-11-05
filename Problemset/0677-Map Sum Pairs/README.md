# 677. Map Sum Pairs
Design a map that allows you to do the following:

* Maps a string key to a given value.
* Returns the sum of the values that have a key with a prefix equal to a given string.

Implement the `MapSum` class:

* `MapSum()` Initializes the `MapSum` object.
* `void insert(String key, int val)` Inserts the `key-val` pair into the map. If the `key` already existed, the original `key-value` pair will be overridden to the new one.
* `int sum(string prefix)` Returns the sum of all the pairs' value whose `key` starts with the `prefix`.

#### Example 1:
<pre>
<strong>Input:</strong>
["MapSum", "insert", "sum", "insert", "sum"]
[[], ["apple", 3], ["ap"], ["app", 2], ["ap"]]
<strong>Output:</strong>
[null, null, 3, null, 5]
<strong>Explanation:</strong>
MapSum mapSum = new MapSum();
mapSum.insert("apple", 3);
mapSum.sum("ap");           // return 3 (apple = 3)
mapSum.insert("app", 2);
mapSum.sum("ap");           // return 5 (apple + app = 3 + 2 = 5)
</pre>

#### Constraints:
* `1 <= key.length, prefix.length <= 50`
* `key` and `prefix` consist of only lowercase English letters.
* `1 <= val <= 1000`
* At most `50` calls will be made to `insert` and `sum`.

## Solutions (Python)

### 1. Solution
```Python
class MapSum:

    def __init__(self):
        self.keyval = {}
        self.trie = {}

    def insert(self, key: str, val: int) -> None:
        diff = val - self.keyval.get(key, 0)
        self.keyval[key] = val
        curr = self.trie

        for ch in key:
            if ch not in curr:
                curr[ch] = {"val": 0}
            curr = curr[ch]
            curr["val"] += diff

    def sum(self, prefix: str) -> int:
        curr = self.trie

        for i, ch in enumerate(prefix):
            if ch not in curr:
                return 0
            curr = curr[ch]
            if i == len(prefix) - 1:
                return curr["val"]


# Your MapSum object will be instantiated and called as such:
# obj = MapSum()
# obj.insert(key,val)
# param_2 = obj.sum(prefix)
```
