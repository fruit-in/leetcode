# 981. 基于时间的键值存储
设计一个基于时间的键值数据结构，该结构可以在不同时间戳存储对应同一个键的多个值，并针对特定时间戳检索键对应的值。

实现 `TimeMap` 类：
* `TimeMap()` 初始化数据结构对象
* `void set(String key, String value, int timestamp)` 存储键 `key`、值 `value`，以及给定的时间戳 `timestamp`。
* `String get(String key, int timestamp)`
    * 返回先前调用 `set(key, value, timestamp_prev)` 所存储的值，其中 `timestamp_prev <= timestamp` 。
    * 如果有多个这样的值，则返回对应最大的  `timestamp_prev` 的那个值。
    * 如果没有值，则返回空字符串（`""`）。

#### 示例:
<pre>
<strong>输入:</strong>
["TimeMap", "set", "get", "get", "set", "get", "get"]
[[], ["foo", "bar", 1], ["foo", 1], ["foo", 3], ["foo", "bar2", 4], ["foo", 4], ["foo", 5]]
<strong>输出:</strong>
[null, null, "bar", "bar", null, "bar2", "bar2"]
<strong>解释:</strong>
TimeMap timeMap = new TimeMap();
timeMap.set("foo", "bar", 1);  // 存储键 "foo" 和值 "bar" ，时间戳 timestamp = 1
timeMap.get("foo", 1);         // 返回 "bar"
timeMap.get("foo", 3);         // 返回 "bar", 因为在时间戳 3 和时间戳 2 处没有对应 "foo" 的值，所以唯一的值位于时间戳 1 处（即 "bar"） 。
timeMap.set("foo", "bar2", 4); // 存储键 "foo" 和值 "bar2" ，时间戳 timestamp = 4
timeMap.get("foo", 4);         // 返回 "bar2"
timeMap.get("foo", 5);         // 返回 "bar2"
</pre>

#### 提示:
* `1 <= key.length, value.length <= 100`
* `key` 和 `value` 由小写英文字母和数字组成
* <code>1 <= timestamp <= 10<sup>7</sup></code>
* `set` 操作中的时间戳 `timestamp` 都是严格递增的
* 最多调用 `set` 和 `get` 操作 <code>2 * 10<sup>5</sup></code> 次

## 题解 (Python)

### 1. 题解
```Python
class TimeMap:

    def __init__(self):
        self.timemap = {}

    def set(self, key: str, value: str, timestamp: int) -> None:
        if key not in self.timemap:
            self.timemap[key] = []
        self.timemap[key].append((timestamp, value))

    def get(self, key: str, timestamp: int) -> str:
        if key not in self.timemap:
            return ""

        i = bisect_right(self.timemap[key], (timestamp, "{")) - 1

        return self.timemap[key][i][1] if i >= 0 else ""


# Your TimeMap object will be instantiated and called as such:
# obj = TimeMap()
# obj.set(key,value,timestamp)
# param_2 = obj.get(key,timestamp)
```
