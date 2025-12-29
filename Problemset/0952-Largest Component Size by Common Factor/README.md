# 952. Largest Component Size by Common Factor
You are given an integer array of unique positive integers `nums`. Consider the following graph:
* There are `nums.length` nodes, labeled `nums[0]` to `nums[nums.length - 1]`,
* There is an undirected edge between `nums[i]` and `nums[j]` if `nums[i]` and `nums[j]` share a common factor greater than `1`.

Return *the size of the largest connected component in the graph*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2018/12/01/ex1.png)
<pre>
<strong>Input:</strong> nums = [4,6,15,35]
<strong>Output:</strong> 4
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2018/12/01/ex2.png)
<pre>
<strong>Input:</strong> nums = [20,50,9,63]
<strong>Output:</strong> 2
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2018/12/01/ex3.png)
<pre>
<strong>Input:</strong> nums = [2,3,6,7,4,12,21,39]
<strong>Output:</strong> 8
</pre>

#### Constraints:
* <code>1 <= nums.length <= 2 * 10<sup>4</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>
* All the values of `nums` are **unique**.

## Solutions (Python)

### 1. Solution
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
