class Solution:
    def superpalindromesInRange(self, left: str, right: str) -> int:
        left = int(left)
        right = int(right)
        ret = 0

        for leftpart in range(1, right + 1):
            s = str(leftpart)
            palindrome = int(s + s[::-1])
            superpalindrome = palindrome * palindrome
            if left <= superpalindrome <= right and str(superpalindrome) == str(superpalindrome)[::-1]:
                ret += 1
            elif superpalindrome < left:
                continue
            palindrome = int(s + s[-2::-1])
            superpalindrome = palindrome * palindrome
            if left <= superpalindrome <= right and str(superpalindrome) == str(superpalindrome)[::-1]:
                ret += 1
            elif superpalindrome > right:
                break

        return ret
