# 2121. 相同元素的间隔之和
给你一个下标从 **0** 开始、由 `n` 个整数组成的数组 `arr` 。

`arr` 中两个元素的 **间隔** 定义为它们下标之间的 **绝对差** 。更正式地，`arr[i]` 和 `arr[j]` 之间的间隔是 `|i - j|` 。

返回一个长度为 `n` 的数组 `intervals` ，其中 `intervals[i]` 是 `arr[i]` 和 `arr` 中每个相同元素（与 `arr[i]` 的值相同）的 **间隔之和** 。

**注意：**`|x|` 是 `x` 的绝对值。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [2,1,3,1,2,3,3]
<strong>输出:</strong> [4,2,7,2,4,4,5]
<strong>解释:</strong>
- 下标 0 ：另一个 2 在下标 4 ，|0 - 4| = 4
- 下标 1 ：另一个 1 在下标 3 ，|1 - 3| = 2
- 下标 2 ：另两个 3 在下标 5 和 6 ，|2 - 5| + |2 - 6| = 7
- 下标 3 ：另一个 1 在下标 1 ，|3 - 1| = 2
- 下标 4 ：另一个 2 在下标 0 ，|4 - 0| = 4
- 下标 5 ：另两个 3 在下标 2 和 6 ，|5 - 2| + |5 - 6| = 4
- 下标 6 ：另两个 3 在下标 2 和 5 ，|6 - 2| + |6 - 5| = 5
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [10,5,10,10]
<strong>输出:</strong> [5,0,3,4]
<strong>解释:</strong>
- 下标 0 ：另两个 10 在下标 2 和 3 ，|0 - 2| + |0 - 3| = 5
- 下标 1 ：只有这一个 5 在数组中，所以到相同元素的间隔之和是 0
- 下标 2 ：另两个 10 在下标 0 和 3 ，|2 - 0| + |2 - 3| = 3
- 下标 3 ：另两个 10 在下标 0 和 2 ，|3 - 0| + |3 - 2| = 4
</pre>

#### 提示:
* `n == arr.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= arr[i] <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def getDistances(self, arr: List[int]) -> List[int]:
        indices_sums = {}
        indices = [0] * len(arr)
        intervals = [0] * len(arr)

        for i in range(len(arr)):
            if arr[i] in indices_sums:
                indices_sums[arr[i]].append(i + indices_sums[arr[i]][-1])
            else:
                indices_sums[arr[i]] = [i]
            indices[i] = len(indices_sums[arr[i]]) - 1

        for i in range(len(arr)):
            prefix_sum = indices_sums[arr[i]]
            intervals[i] = prefix_sum[-1] + 2 * i * indices[i] + 2 * i \
                - 2 * prefix_sum[indices[i]] - i * len(prefix_sum)

        return intervals
```
