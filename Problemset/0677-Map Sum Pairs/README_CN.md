# 677. 键值映射
设计一个 map ，满足以下几点:

* 字符串表示键，整数表示值
* 返回具有前缀等于给定字符串的键的值的总和

实现一个 `MapSum` 类：

* `MapSum()` 初始化 `MapSum` 对象
* `void insert(String key, int val)` 插入 `key-val` 键值对，字符串表示键 `key` ，整数表示值 `val` 。如果键 `key` 已经存在，那么原来的键值对 `key-value` 将被替代成新的键值对。
* `int sum(string prefix)` 返回所有以该前缀 `prefix` 开头的键 `key` 的值的总和。

#### 示例 1:
<pre>
<strong>输入:</strong>
["MapSum", "insert", "sum", "insert", "sum"]
[[], ["apple", 3], ["ap"], ["app", 2], ["ap"]]
<strong>输出:</strong>
[null, null, 3, null, 5]
<strong>解释:</strong>
MapSum mapSum = new MapSum();
mapSum.insert("apple", 3);
mapSum.sum("ap");           // 返回 3 (apple = 3)
mapSum.insert("app", 2);
mapSum.sum("ap");           // 返回 5 (apple + app = 3 + 2 = 5)
</pre>

#### 提示:
* `1 <= key.length, prefix.length <= 50`
* `key` 和 `prefix` 仅由小写英文字母组成
* `1 <= val <= 1000`
* 最多调用 `50` 次 `insert` 和 `sum`

## 题解 (Python)

### 1. 题解
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
