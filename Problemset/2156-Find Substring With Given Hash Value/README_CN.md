# 2156. 查找给定哈希值的子串
给定整数 `p` 和 `m` ，一个长度为 `k` 且下标从 **0** 开始的字符串 `s` 的哈希值按照如下函数计算：
* <code>hash(s, p, m) = (val(s[0]) * p<sup>0</sup> + val(s[1]) * p<sup>1</sup> + ... + val(s[k-1]) * p<sup>k-1</sup>) mod m</code>.

其中 `val(s[i])` 表示 `s[i]` 在字母表中的下标，从 `val('a') = 1` 到 `val('z') = 26` 。

给你一个字符串 `s` 和整数 `power`，`modulo`，`k` 和 `hashValue` 。请你返回 `s` 中 **第一个** 长度为 `k` 的 **子串** `sub` ，满足 `hash(sub, power, modulo) == hashValue` 。

测试数据保证一定 **存在** 至少一个这样的子串。

**子串** 定义为一个字符串中连续非空字符组成的序列。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "leetcode", power = 7, modulo = 20, k = 2, hashValue = 0
<strong>输出:</strong> "ee"
<strong>解释:</strong> "ee" 的哈希值为 hash("ee", 7, 20) = (5 * 1 + 5 * 7) mod 20 = 40 mod 20 = 0 。
"ee" 是长度为 2 的第一个哈希值为 0 的子串，所以我们返回 "ee" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "fbxzaad", power = 31, modulo = 100, k = 3, hashValue = 32
<strong>输出:</strong> "fbx"
<strong>解释:</strong> "fbx" 的哈希值为 hash("fbx", 31, 100) = (6 * 1 + 2 * 31 + 24 * 312) mod 100 = 23132 mod 100 = 32 。
"bxz" 的哈希值为 hash("bxz", 31, 100) = (2 * 1 + 24 * 31 + 26 * 312) mod 100 = 25732 mod 100 = 32 。
"fbx" 是长度为 3 的第一个哈希值为 32 的子串，所以我们返回 "fbx" 。
注意，"bxz" 的哈希值也为 32 ，但是它在字符串中比 "fbx" 更晚出现。
</pre>

#### 提示:
* <code>1 <= k <= s.length <= 2 * 10<sup>4</sup></code>
* <code>1 <= power, modulo <= 10<sup>9</sup></code>
* `0 <= hashValue < modulo`
* `s` 只包含小写英文字母。
* 测试数据保证一定 **存在** 满足条件的子串。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def subStrHash(self, s: str, power: int, modulo: int, k: int, hashValue: int) -> str:
        def val(c): return ord(c) - 96
        value = 0
        start = 0

        for i in range(len(s) - 1, -1, -1):
            value = (value * power + val(s[i])) % modulo
            if i + k < len(s):
                value = (value - val(s[i + k]) *
                         pow(power, k, modulo)) % modulo
            if value == hashValue:
                start = i

        return s[start:start + k]
```
