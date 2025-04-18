# 1095. 山脉数组中查找目标值
（这是一个 **交互式问题** ）

你可以将一个数组 `arr` 称为 **山脉数组** 当且仅当：
* `arr.length >= 3`
* 存在一些 `0 < i < arr.length - 1` 的 `i` 使得：
    * `arr[0] < arr[1] < ... < arr[i - 1] < arr[i]`
    * `arr[i] > arr[i + 1] > ... > arr[arr.length - 1]`

给定一个山脉数组 `mountainArr` ，返回 **最小** 的 `index` 使得 `mountainArr.get(index) == target`。如果不存在这样的 `index`，返回 `-1` 。

**你无法直接访问山脉数组**。你只能使用 `MountainArray` 接口来访问数组：
* `MountainArray.get(k)` 返回数组中下标为 `k` 的元素（从 0 开始）。
* `MountainArray.length()` 返回数组的长度。

调用 `MountainArray.get` 超过 `100` 次的提交会被判定为错误答案。此外，任何试图绕过在线评测的解决方案都将导致取消资格。

#### 示例 1:
<pre>
<strong>输入:</strong> mountainArr = [1,2,3,4,5,3,1], target = 3
<strong>输出:</strong> 2
<strong>解释:</strong> 3 在数组中出现了两次，下标分别为 2 和 5，我们返回最小的下标 2。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> mountainArr = [0,1,2,4,2,1], target = 3
<strong>输出:</strong> -1
<strong>解释:</strong> 3 在数组中没有出现，返回 -1。
</pre>

#### 提示:
* <code>3 <= mountainArr.length() <= 10<sup>4</sup></code>
* <code>0 <= target <= 10<sup>9</sup></code>
* <code>0 <= mountainArr.get(index) <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
# """
# This is MountainArray's API interface.
# You should not implement it, or speculate about its implementation
# """
# class MountainArray:
#    def get(self, index: int) -> int:
#    def length(self) -> int:

class Solution:
    def findInMountainArray(self, target: int, mountainArr: 'MountainArray') -> int:
        n = mountainArr.length()
        i = bisect.bisect(range(n - 1), False, key=lambda j: mountainArr.get(
            j) > mountainArr.get(j + 1))

        index = bisect.bisect_left(range(n), target, hi=i, key=mountainArr.get)
        if mountainArr.get(index) == target:
            return index
        index = bisect.bisect_left(
            range(n - 1), -target, lo=i, key=lambda j: -mountainArr.get(j))
        if mountainArr.get(index) == target:
            return index

        return -1
```
