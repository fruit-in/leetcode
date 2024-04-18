# 432. All O`one Data Structure
Design a data structure to store the strings' count with the ability to return the strings with minimum and maximum counts.

Implement the `AllOne` class:

* `AllOne()` Initializes the object of the data structure.
* `inc(String key)` Increments the count of the string `key` by `1`. If `key` does not exist in the data structure, insert it with count `1`.
* `dec(String key)` Decrements the count of the string `key` by `1`. If the count of `key` is `0` after the decrement, remove it from the data structure. It is guaranteed that `key` exists in the data structure before the decrement.
* `getMaxKey()` Returns one of the keys with the maximal count. If no element exists, return an empty string `""`.
* `getMinKey()` Returns one of the keys with the minimum count. If no element exists, return an empty string `""`.

**Note** that each function must run in `O(1)` average time complexity.

#### Example 1:
<pre>
<strong>Input:</strong>
["AllOne", "inc", "inc", "getMaxKey", "getMinKey", "inc", "getMaxKey", "getMinKey"]
[[], ["hello"], ["hello"], [], [], ["leet"], [], []]
<strong>Output:</strong>
[null, null, null, "hello", "hello", null, "hello", "leet"]
<strong>Explanation:</strong>
AllOne allOne = new AllOne();
allOne.inc("hello");
allOne.inc("hello");
allOne.getMaxKey(); // return "hello"
allOne.getMinKey(); // return "hello"
allOne.inc("leet");
allOne.getMaxKey(); // return "hello"
allOne.getMinKey(); // return "leet"
</pre>

#### Constraints:
* `1 <= key.length <= 10`
* `key` consists of lowercase English letters.
* It is guaranteed that for each call to `dec`, `key` is existing in the data structure.
* At most <code>5 * 10<sup>4</sup></code> calls will be made to `inc`, `dec`, `getMaxKey`, and `getMinKey`.

## Solutions (Python)

### 1. Solution
```Python
class AllOne:

    def __init__(self):
        self.keys = []
        self.count = {}
        self.index = {}
        self.range = {}

    def inc(self, key: str) -> None:
        if key not in self.count:
            self.keys.append(key)
            self.count[key] = 1
            self.index[key] = len(self.keys) - 1
            if 1 not in self.range:
                self.range[1] = [self.index[key], self.index[key]]
            self.range[1][1] = self.index[key]
        else:
            count0 = self.count[key]
            count1 = count0 + 1
            i = self.range[count0][0]
            j = self.index[key]
            self.index[self.keys[i]], self.index[self.keys[j]] = j, i
            self.keys[i], self.keys[j] = self.keys[j], self.keys[i]
            self.count[key] += 1
            self.range[count0][0] += 1
            if self.range[count0][0] > self.range[count0][1]:
                self.range.pop(count0)
            if count1 not in self.range:
                self.range[count1] = [i, i]
            self.range[count1][1] = i

    def dec(self, key: str) -> None:
        if self.count[key] == 1:
            self.index[self.keys[-1]] = self.index[key]
            self.keys[self.index[key]] = self.keys[-1]
            self.keys.pop()
            self.count.pop(key)
            self.index.pop(key)
            self.range[1][1] -= 1
            if self.range[1][0] > self.range[1][1]:
                self.range.pop(1)
        else:
            count0 = self.count[key]
            count1 = count0 - 1
            i = self.range[count0][1]
            j = self.index[key]
            self.index[self.keys[i]], self.index[self.keys[j]] = j, i
            self.keys[i], self.keys[j] = self.keys[j], self.keys[i]
            self.count[key] -= 1
            self.range[count0][1] -= 1
            if self.range[count0][0] > self.range[count0][1]:
                self.range.pop(count0)
            if count1 not in self.range:
                self.range[count1] = [i, i]
            self.range[count1][0] = i

    def getMaxKey(self) -> str:
        return self.keys[0] if self.keys != [] else ""

    def getMinKey(self) -> str:
        return self.keys[-1] if self.keys != [] else ""


# Your AllOne object will be instantiated and called as such:
# obj = AllOne()
# obj.inc(key)
# obj.dec(key)
# param_3 = obj.getMaxKey()
# param_4 = obj.getMinKey()
```
