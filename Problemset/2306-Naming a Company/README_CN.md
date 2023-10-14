# 2306. 公司命名
给你一个字符串数组 `ideas` 表示在公司命名过程中使用的名字列表。公司命名流程如下：

1. 从 `ideas` 中选择 2 个 **不同** 名字，称为 <code>idea<sub>A</sub></code> 和 <code>idea<sub>B</sub></code> 。
2. 交换 <code>idea<sub>A</sub></code> 和 <code>idea<sub>B</sub></code> 的首字母。
3. 如果得到的两个新名字 **都** 不在 `ideas` 中，那么 <code>idea<sub>A</sub></code> <code>idea<sub>B</sub></code>（**串联** <code>idea<sub>A</sub></code> 和 <code>idea<sub>B</sub></code> ，中间用一个空格分隔）是一个有效的公司名字。
4. 否则，不是一个有效的名字。

返回 **不同** 且有效的公司名字的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> ideas = ["coffee","donuts","time","toffee"]
<strong>输出:</strong> 6
<strong>解释:</strong> 下面列出一些有效的选择方案：
- ("coffee", "donuts")：对应的公司名字是 "doffee conuts" 。
- ("donuts", "coffee")：对应的公司名字是 "conuts doffee" 。
- ("donuts", "time")：对应的公司名字是 "tonuts dime" 。
- ("donuts", "toffee")：对应的公司名字是 "tonuts doffee" 。
- ("time", "donuts")：对应的公司名字是 "dime tonuts" 。
- ("toffee", "donuts")：对应的公司名字是 "doffee tonuts" 。
因此，总共有 6 个不同的公司名字。

下面列出一些无效的选择方案：
- ("coffee", "time")：在原数组中存在交换后形成的名字 "toffee" 。
- ("time", "toffee")：在原数组中存在交换后形成的两个名字。
- ("coffee", "toffee")：在原数组中存在交换后形成的两个名字。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> ideas = ["lack","back"]
<strong>输出:</strong> 0
<strong>解释:</strong> 不存在有效的选择方案。因此，返回 0 。
</pre>

#### 提示:
* <code>2 <= ideas.length <= 5 * 10<sup>4</sup></code>
* `1 <= ideas[i].length <= 10`
* `ideas[i]` 由小写英文字母组成
* `ideas` 中的所有字符串 **互不相同**

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def distinctNames(self, ideas: List[str]) -> int:
        firsts = [set() for _ in range(26)]
        ret = 0

        for idea in ideas:
            firsts[ord(idea[0]) - 97].add(idea[1:])

        for i in range(26):
            for j in range(i + 1, 26):
                intersectionlen = len(firsts[i] & firsts[j])
                ret += (len(firsts[i]) - intersectionlen) * \
                    (len(firsts[j]) - intersectionlen) * 2

        return ret
```
