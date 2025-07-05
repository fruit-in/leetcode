# 1157. 子数组中占绝大多数的元素
设计一个数据结构，有效地找到给定子数组的 **多数元素** 。

子数组的 **多数元素** 是在子数组中出现 `threshold` 次数或次数以上的元素。

实现 `MajorityChecker` 类:
* `MajorityChecker(int[] arr)` 会用给定的数组 `arr` 对 `MajorityChecker` 初始化。
* `int query(int left, int right, int threshold)` 返回子数组中的元素  `arr[left...right]` 至少出现 `threshold` 次数，如果不存在这样的元素则返回 `-1`。

#### 示例 1:
<pre>
<strong>输入:</strong>
["MajorityChecker", "query", "query", "query"]
[[[1, 1, 2, 2, 1, 1]], [0, 5, 4], [0, 3, 3], [2, 3, 2]]
<strong>输出:</strong>
[null, 1, -1, 2]
<strong>解释:</strong>
MajorityChecker majorityChecker = new MajorityChecker([1,1,2,2,1,1]);
majorityChecker.query(0,5,4); // 返回 1
majorityChecker.query(0,3,3); // 返回 -1
majorityChecker.query(2,3,2); // 返回 2
</pre>

#### 提示:
* <code>1 <= arr.length <= 2 * 10<sup>4</sup></code>
* <code>1 <= arr[i] <= 2 * 10<sup>4</sup></code>
* `0 <= left <= right < arr.length`
* `threshold <= right - left + 1`
* `2 * threshold > right - left + 1`
* 调用 `query` 的次数最多为 <code>10<sup>4</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class MajorityChecker:

    def __init__(self, arr: List[int]):
        self.arr = arr
        self.indices = {}

        for i, x in enumerate(arr):
            if x not in self.indices:
                self.indices[x] = []
            self.indices[x].append(i)

    def query(self, left: int, right: int, threshold: int) -> int:
        used = set()
        count = 0

        while right - left + 1 - count >= threshold:
            x = self.arr[randint(left, right)]
            while x in used:
                x = self.arr[randint(left, right)]
            used.add(x)
            indices = self.indices[x]
            occur = bisect_right(indices, right) - bisect_left(indices, left)
            count += occur
            if occur >= threshold:
                return x

        return -1


# Your MajorityChecker object will be instantiated and called as such:
# obj = MajorityChecker(arr)
# param_1 = obj.query(left,right,threshold)
```
