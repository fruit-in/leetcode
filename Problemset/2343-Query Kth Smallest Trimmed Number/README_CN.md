# 2343. 裁剪数字后查询第 K 小的数字
给你一个下标从 **0** 开始的字符串数组 `nums` ，其中每个字符串 **长度相等** 且只包含数字。

再给你一个下标从 **0** 开始的二维整数数组 `queries` ，其中 <code>queries[i] = [k<sub>i</sub>, trim<sub>i</sub>]</code> 。对于每个 `queries[i]` ，你需要：

* 将 `nums` 中每个数字 **裁剪** 到剩下 **最右边** <code>trim<sub>i</sub></code> 个数位。
* 在裁剪过后的数字中，找到 `nums` 中第 <code>k<sub>i</sub></code> 小数字对应的 **下标** 。如果两个裁剪后数字一样大，那么下标 **更小** 的数字视为更小的数字。
* 将 `nums` 中每个数字恢复到原本字符串。

请你返回一个长度与 `queries` 相等的数组 `answer`，其中 `answer[i]`是第 `i` 次查询的结果。

**提示：**

* 裁剪到剩下最右边 `x` 个数位的意思是不断删除最左边的数位，直到剩下 `x` 个数位。
* `nums` 中的字符串可能会有前导 0 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = ["102","473","251","814"], queries = [[1,1],[2,3],[4,2],[1,2]]
<strong>输出:</strong> [2,2,1,0]
<strong>解释:</strong>
1. 裁剪到只剩 1 个数位后，nums = ["2","3","1","4"] 。最小的数字是 1 ，下标为 2 。
2. 裁剪到剩 3 个数位后，nums 没有变化。第 2 小的数字是 251 ，下标为 2 。
3. 裁剪到剩 2 个数位后，nums = ["02","73","51","14"] 。第 4 小的数字是 73 ，下标为 1 。
4. 裁剪到剩 2 个数位后，最小数字是 2 ，下标为 0 。
   注意，裁剪后数字 "02" 值为 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = ["24","37","96","04"], queries = [[2,1],[2,2]]
<strong>输出:</strong> [3,0]
<strong>解释:</strong>
1. 裁剪到剩 1 个数位，nums = ["4","7","6","4"] 。第 2 小的数字是 4 ，下标为 3 。
   有两个 4 ，下标为 0 的 4 视为小于下标为 3 的 4 。
2. 裁剪到剩 2 个数位，nums 不变。第二小的数字是 24 ，下标为 0 。
</pre>

#### 提示:
* `1 <= nums.length <= 100`
* `1 <= nums[i].length <= 100`
* `nums[i]` 只包含数字。
* 所有 `nums[i].length` 的长度 **相同** 。
* `1 <= queries.length <= 100`
* `queries[i].length == 2`
* <code>1 <= k<sub>i</sub> <= nums.length</code>
* <code>1 <= trim<sub>i</sub> <= nums[i].length</code>

**进阶：**你能使用 **基数排序算法** 解决此问题吗？这种解法的复杂度又是多少？

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def smallestTrimmedNumbers(self, nums: List[str], queries: List[List[int]]) -> List[int]:
        nums = [(nums[i], i) for i in range(len(nums))]
        trims = {}
        answer = [0] * len(queries)

        for i in range(len(queries)):
            if queries[i][1] not in trims:
                trims[queries[i][1]] = []
            trims[queries[i][1]].append((i, queries[i][0] - 1))

        for trim in range(1, len(nums[0][0]) + 1):
            count = [0] * 10
            tmp = [("", 0)] * len(nums)

            for num, _ in nums:
                count[ord(num[-trim]) - 48] += 1
            for i in range(1, 10):
                count[i] += count[i - 1]
            for num, i in nums[::-1]:
                count[ord(num[-trim]) - 48] -= 1
                tmp[count[ord(num[-trim]) - 48]] = (num, i)

            for i, k in trims.get(trim, []):
                answer[i] = tmp[k][1]

            nums = tmp

        return answer
```
