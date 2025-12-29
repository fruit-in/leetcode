# 952. 按公因数计算最大组件大小
给定一个由不同正整数的组成的非空数组 `nums` ，考虑下面的图：
* 有 `nums.length` 个节点，按从 `nums[0]` 到 `nums[nums.length - 1]` 标记；
* 只有当 `nums[i]` 和 `nums[j]` 共用一个大于 1 的公因数时，`nums[i]` 和 `nums[j]`之间才有一条边。

返回 *图中最大连通组件的大小* 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2018/12/01/ex1.png)
<pre>
<strong>输入:</strong> nums = [4,6,15,35]
<strong>输出:</strong> 4
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2018/12/01/ex2.png)
<pre>
<strong>输入:</strong> nums = [20,50,9,63]
<strong>输出:</strong> 2
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2018/12/01/ex3.png)
<pre>
<strong>输入:</strong> nums = [2,3,6,7,4,12,21,39]
<strong>输出:</strong> 8
</pre>

#### 提示:
* <code>1 <= nums.length <= 2 * 10<sup>4</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>
* `nums` 中所有值都 **不同**

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    primefactors = [[] for _ in range(100001)]

    for i in range(2, 100001):
        if not primefactors[i]:
            for j in range(i, 100001, i):
                primefactors[j].append(i)

    def largestComponentSize(self, nums: List[int]) -> int:
        parent = list(range(max(nums) + 1))
        count = {}

        for i in nums:
            while parent[i] != parent[parent[i]]:
                parent[i] = parent[parent[i]]
            count[parent[i]] = count.get(parent[i], 0) + 1

            for j in self.primefactors[i]:
                while parent[j] != parent[parent[j]]:
                    parent[j] = parent[parent[j]]
                if parent[j] != parent[i]:
                    count[parent[i]] += count.get(parent[j], 0)
                    parent[parent[j]] = parent[i]

        return max(count.values())
```
