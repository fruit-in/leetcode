class Solution:
    def isValid(self, s: str) -> bool:
        if not s:
            return True
        if len(s) % 2 != 0:
            return False
        brackets = {')': '(', '}': '{', ']': '['}
        stack = []
        for i in s:
            if i in brackets.values():
                stack.append(i)
            elif i in brackets and stack and stack[-1] == brackets[i]:
                stack = stack[:-1]
            else:
                return False
        if stack:
            return False
        else:
            return True
