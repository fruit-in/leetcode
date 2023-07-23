class Solution:
    def validPalindrome(self, s: str) -> bool:
        def isPalindrome(l: int, r: int) -> bool:
            return all(s[l + i] == s[r - i] for i in range((r - l + 1) // 2))

        l, r = 0, len(s) - 1
        while l < r:
            if s[l] != s[r]:
                return isPalindrome(l + 1, r) or isPalindrome(l, r - 1)

            l += 1
            r -= 1

        return True
