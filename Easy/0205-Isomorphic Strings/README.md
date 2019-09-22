# 205. Isomorphic Strings
Given two strings ***s*** and ***t***, determine if they are isomorphic.

Two strings are isomorphic if the characters in ***s*** can be replaced to get ***t***.

All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character but a character may map to itself.

#### Example 1:
<pre>
<strong>Input:</strong> <strong><em>s</em></strong> = "egg", <strong><em>t</em></strong> = "add"
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> <strong><em>s</em></strong> = "foo", <strong><em>t</em></strong> = "bar"
<strong>Output:</strong> false
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> <strong><em>s</em></strong> = "paper", <strong><em>t</em></strong> = "title"
<strong>Output:</strong> true
</pre>

#### Note:
You may assume both ***s*** and ***t*** have the same length.

## Solutions (Python)

### 1. Brute Force
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

### 2. HashMap
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
