class Solution:
    def decodeString(self, s: str) -> str:
        stack = []
        k = 0

        for c in s:
            if c.isdigit():
                k = k * 10 + int(c)
            elif c == '[':
                stack.append(str(k))
                k = 0
            elif c.islower():
                stack.append(c)
            elif c == ']':
                tmp = []
                while stack[-1].islower():
                    tmp.append(stack.pop())
                stack.append(''.join(tmp[::-1]) * int(stack.pop()))

        return ''.join(stack)
