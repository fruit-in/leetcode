# 969. 煎饼排序
给定数组 ```A```，我们可以对其进行煎饼翻转：我们选择一些正整数 <code><strong>k</strong> <= A.length</code>然后反转 ```A``` 的前 **k** 个元素的顺序。我们要执行零次或多次煎饼翻转（按顺序一次接一次地进行）以完成对数组 ```A``` 的排序。

返回能使 ```A``` 排序的煎饼翻转操作所对应的 k 值序列。任何将数组排序且翻转次数在 ```10 * A.length``` 范围内的有效答案都将被判断为正确。

#### 示例 1:
<pre>
<strong>输入:</strong> [3,2,4,1]
<strong>输出:</strong> [4,2,4,3]
<strong>解释:</strong>
我们执行 4 次煎饼翻转，k 值分别为 4，2，4，和 3。
初始状态 A = [3, 2, 4, 1]
第一次翻转后 (k=4): A = [1, 4, 2, 3]
第二次翻转后 (k=2): A = [4, 1, 2, 3]
第三次翻转后 (k=4): A = [3, 2, 1, 4]
第四次翻转后 (k=3): A = [1, 2, 3, 4]，此时已完成排序。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [1,2,3]
<strong>输出:</strong> []
<strong>解释:</strong>
输入已经排序，因此不需要翻转任何内容。
请注意，其他可能的答案，如[3，3]，也将被接受。
</pre>

#### 提示:
1. <code>1 <= A.length <= 100</code>
2. <code>A[i]</code> 是 <code>[1, 2, ..., A.length]</code> 的排列

## 题解 (Python)

### 1. 先翻转至数组首位，再翻转至正确位置
```Python3
class Solution:
    def pancakeSort(self, A: List[int]) -> List[int]:
        finalA = sorted(A)
        ks = []
        n = len(A)
        for n in range(len(A), 0, -1):
            if A == finalA:
                break
            index = A.index(n)
            if index == n - 1:
                continue
            if index != 0:
                A[:index + 1] = A[index::-1]
                ks.append(index + 1)
            A[:n] = A[n - 1::-1]
            ks.append(n)
        return ks
```
