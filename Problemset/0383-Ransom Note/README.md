# 383. Ransom Note
Given an arbitrary ransom note string and another string containing letters from all the magazines, write a function that will return true if the ransom note can be constructed from the magazines ; otherwise, it will return false.

Each letter in the magazine string can only be used once in your ransom note.

#### Note:
You may assume that both strings contain only lowercase letters.

```
canConstruct("a", "b") -> false
canConstruct("aa", "ab") -> false
canConstruct("aa", "aab") -> true
```

## Solutions (Python)

### 1. Solution
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
