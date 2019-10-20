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
