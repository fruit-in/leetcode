# 383. 赎金信
给定一个赎金信 (ransom) 字符串和一个杂志(magazine)字符串，判断第一个字符串ransom能不能由第二个字符串magazines里面的字符构成。如果可以构成，返回 true ；否则返回 false。

(题目说明：为了不暴露赎金信字迹，要从杂志上搜索各个需要的字母，组成单词来表达意思。)

#### 注意:
你可以假设两个字符串均只含有小写字母。

```
canConstruct("a", "b") -> false
canConstruct("aa", "ab") -> false
canConstruct("aa", "aab") -> true
```

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def canConstruct(self, ransomNote: str, magazine: str) -> bool:
        cnt = [0] * 26

        for ch in magazine:
            cnt[ord(ch) - 97] += 1

        for ch in ransomNote:
            if cnt[ord(ch) - 97] == 0:
                return False
            cnt[ord(ch) - 97] -= 1

        return True
```
