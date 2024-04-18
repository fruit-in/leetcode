# 432. 全 O(1) 的数据结构
请你设计一个用于存储字符串计数的数据结构，并能够返回计数最小和最大的字符串。

实现 `AllOne` 类：

* `AllOne()` 初始化数据结构的对象。
* `inc(String key)` 字符串 `key` 的计数增加 `1` 。如果数据结构中尚不存在 `key` ，那么插入计数为 `1` 的 `key` 。
* `dec(String key)` 字符串 `key` 的计数减少 `1` 。如果 `key` 的计数在减少后为 `0` ，那么需要将这个 `key` 从数据结构中删除。测试用例保证：在减少计数前，`key` 存在于数据结构中。
* `getMaxKey()` 返回任意一个计数最大的字符串。如果没有元素存在，返回一个空字符串 `""` 。
* `getMinKey()` 返回任意一个计数最小的字符串。如果没有元素存在，返回一个空字符串 `""` 。

注意：每个函数都应当满足 `O(1)` 平均时间复杂度。

#### 示例 1:
<pre>
<strong>输入:</strong>
["AllOne", "inc", "inc", "getMaxKey", "getMinKey", "inc", "getMaxKey", "getMinKey"]
[[], ["hello"], ["hello"], [], [], ["leet"], [], []]
<strong>输出:</strong>
[null, null, null, "hello", "hello", null, "hello", "leet"]
<strong>解释:</strong>
AllOne allOne = new AllOne();
allOne.inc("hello");
allOne.inc("hello");
allOne.getMaxKey(); // 返回 "hello"
allOne.getMinKey(); // 返回 "hello"
allOne.inc("leet");
allOne.getMaxKey(); // 返回 "hello"
allOne.getMinKey(); // 返回 "leet"
</pre>

#### 提示:
* `1 <= key.length <= 10`
* `key` 由小写英文字母组成
* 测试用例保证：在每次调用 `dec` 时，数据结构中总存在 `key`
* 最多调用 `inc`、`dec`、`getMaxKey` 和 `getMinKey` 方法 <code>5 * 10<sup>4</sup></code> 次

## 题解 (Python)

### 1. 题解
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
