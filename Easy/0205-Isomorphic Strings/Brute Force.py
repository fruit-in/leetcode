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
