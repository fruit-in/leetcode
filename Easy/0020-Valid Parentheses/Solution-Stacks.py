class Solution:
    def isValid(self, s: str) -> bool:
        if len(s) % 2 != 0:
            return False
        brackets = {')': '(', '}': '{', ']': '['}
        stack = []
        for c in s:
            if c in brackets and stack and stack[-1] == brackets[c]:
                stack = stack[:-1]
            else:
                stack.append(c)
        return not stack
