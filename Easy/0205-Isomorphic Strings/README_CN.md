# 205. 同构字符串
给定两个字符串 ***s*** 和 ***t***，判断它们是否是同构的。

如果 ***s*** 中的字符可以被替换得到 ***t*** ，那么这两个字符串是同构的。

所有出现的字符都必须用另一个字符替换，同时保留字符的顺序。两个字符不能映射到同一个字符上，但字符可以映射自己本身。

#### 示例 1:
<pre>
<strong>输入:</strong> <strong><em>s</em></strong> = "egg", <strong><em>t</em></strong> = "add"
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> <strong><em>s</em></strong> = "foo", <strong><em>t</em></strong> = "bar"
<strong>输出:</strong> false
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> <strong><em>s</em></strong> = "paper", <strong><em>t</em></strong> = "title"
<strong>输出:</strong> true
</pre>

#### 说明:
你可以假设 ***s*** 和 ***t*** 具有相同的长度。

## 题解 (Python)

### 1. 暴力法
```Python
class Solution:
    def isIsomorphic(self, s: str, t: str) -> bool:
        checked = set()
        for i in range(len(s)):
            if s[i] not in checked:
                for j in range(i + 1, len(s)):
                    if s[j] == s[i] and t[j] != t[i] or s[j] != s[i] and t[j] == t[i]:
                        return False
                checked.add(s[i])
        return True
```

### 2. 哈希表
```Python
class Solution:
    def isIsomorphic(self, s: str, t: str) -> bool:
        smap = {}
        tset = set()

        for cs, ct in zip(s, t):
            if (cs in smap) != (ct in tset):
                return False
            elif cs not in smap:
                smap[cs] = ct
                tset.add(ct)
            elif smap[cs] != ct:
                return False

        return True
```
