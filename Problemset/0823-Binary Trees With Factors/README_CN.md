# 823. 带因子的二叉树
给出一个含有不重复整数元素的数组 `arr` ，每个整数 `arr[i]` 均大于 1。

用这些整数来构建二叉树，每个整数可以使用任意次数。其中：每个非叶结点的值应等于它的两个子结点的值的乘积。

满足条件的二叉树一共有多少个？答案可能很大，返回 **对** <code>10<sup>9</sup> + 7</code> **取余** 的结果。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [2,4]
<strong>输出:</strong> 3
<strong>解释:</strong> 可以得到这些二叉树: [2], [4], [4, 2, 2]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [2,4,5,10]
<strong>输出:</strong> 7
<strong>解释:</strong> 可以得到这些二叉树: [2], [4], [5], [10], [4, 2, 2], [10, 2, 5], [10, 5, 2].
</pre>

#### 提示:
* `1 <= arr.length <= 1000`
* <code>2 <= arr[i] <= 10<sup>9</sup></code>
* `arr` 中的所有值 **互不相同**

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def numFactoredBinaryTrees(self, arr: List[int]) -> int:
        count = {}
        ret = 0

        arr.sort()

        for i in range(len(arr)):
            count[arr[i]] = 1

            for j in range(i):
                if arr[i] % arr[j] == 0:
                    count[arr[i]] = (
                        count[arr[i]] + count[arr[j]] * count.get(arr[i] // arr[j], 0)) % 1_000_000_007

            ret = (ret + count[arr[i]]) % 1_000_000_007

        return ret
```
