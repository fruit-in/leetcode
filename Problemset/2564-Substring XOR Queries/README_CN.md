# 2564. 子字符串异或查询
给你一个 **二进制字符串** `s` 和一个整数数组 `queries` ，其中 <code>queries[i] = [first<sub>i</sub>, second<sub>i</sub>]</code> 。

对于第 `i` 个查询，找到 `s` 的 **最短子字符串** ，它对应的 **十进制**值 `val` 与 <code>first<sub>i</sub></code> **按位异或** 得到 <code>second<sub>i</sub></code> ，换言之，<code>val ^ first<sub>i</sub> == second<sub>i</sub></code> 。

第 `i` 个查询的答案是子字符串 <code>[left<sub>i</sub>, right<sub>i</sub>]</code> 的两个端点（下标从 **0** 开始），如果不存在这样的子字符串，则答案为 `[-1, -1]` 。如果有多个答案，请你选择 <code>left<sub>i</sub></code> 最小的一个。

请你返回一个数组 `ans` ，其中 <code>ans[i] = [left<sub>i</sub>, right<sub>i</sub>]</code> 是第 `i` 个查询的答案。

**子字符串** 是一个字符串中一段连续非空的字符序列。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "101101", queries = [[0,5],[1,2]]
<strong>输出:</strong> [[0,2],[2,3]]
<strong>解释:</strong> 第一个查询，端点为 [0,2] 的子字符串为 "101" ，对应十进制数字 5 ，且 5 ^ 0 = 5 ，所以第一个查询的答案为 [0,2]。第二个查询中，端点为 [2,3] 的子字符串为 "11" ，对应十进制数字 3 ，且 3 ^ 1 = 2 。所以第二个查询的答案为 [2,3] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "0101", queries = [[12,8]]
<strong>输出:</strong> [[-1,-1]]
<strong>解释:</strong> 这个例子中，没有符合查询的答案，所以返回 [-1,-1] 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "1", queries = [[4,5]]
<strong>输出:</strong> [[0,0]]
<strong>解释:</strong> 这个例子中，端点为 [0,0] 的子字符串对应的十进制值为 1 ，且 1 ^ 4 = 5 。所以答案为 [0,0] 。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>4</sup></code>
* `s[i]` 要么是 `'0'` ，要么是 `'1'` 。
* <code>1 <= queries.length <= 10<sup>5</sup></code>
* <code>0 <= first<sub>i</sub>, second<sub>i</sub> <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def substringXorQueries(self, s: str, queries: List[List[int]]) -> List[List[int]]:
        vals = {}

        for i in range(len(s)):
            if s[i] == '0':
                if 0 not in vals:
                    vals[0] = [i, i]
                continue

            val = 0

            for j in range(i, min(len(s), i + 32)):
                val = (val << 1) + int(s[j])
                if val not in vals:
                    vals[val] = [i, j]

        return [vals.get(first ^ second, [-1, -1]) for first, second in queries]
```
