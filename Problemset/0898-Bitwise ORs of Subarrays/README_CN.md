# 898. 子数组按位或操作
给定一个整数数组 `arr`，返回所有 `arr` 的非空子数组的不同按位或的数量。

子数组的按位或是子数组中每个整数的按位或。含有一个整数的子数组的按位或就是该整数。

**子数组** 是数组内连续的非空元素序列。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [0]
<strong>输出:</strong> 1
<strong>解释:</strong>
只有一个可能的结果 0 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,1,2]
<strong>输出:</strong> 3
<strong>解释:</strong>
可能的子数组为 [1]，[1]，[2]，[1, 1]，[1, 2]，[1, 1, 2]。
产生的结果为 1，1，2，1，3，3 。
有三个唯一值，所以答案是 3 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [1,2,4]
<strong>输出:</strong> 6
<strong>解释:</strong>
可能的结果是 1，2，3，4，6，以及 7 。
</pre>

#### 提示:
* <code>1 <= arr.length <= 5 * 10<sup>4</sup></code>
* <code>0 <= arr[i] <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def subarrayBitwiseORs(self, arr: List[int]) -> int:
        subarrayors = set(arr)

        for i in range(len(arr)):
            for j in range(i - 1, -1, -1):
                if arr[j] | arr[i] == arr[j]:
                    break
                arr[j] |= arr[i]
                subarrayors.add(arr[j])

        return len(subarrayors)
```
