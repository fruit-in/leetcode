class Solution:
    def calculate(self, s: str) -> int:
        s = s.replace(' ', '')
        negstack = [False]
        stack = []
        ret = 0

        for i in range(len(s)):
            if s[i] == '(':
                if i == 0 or s[i - 1] != '-':
                    negstack.append(negstack[-1])
                else:
                    negstack.append(not negstack[-1])
            elif s[i] == ')':
                negstack.pop()
            elif s[i] == '+':
                stack.append('-' if negstack[-1] else '+')
            elif s[i] == '-':
                if stack == [] or isinstance(stack[-1], str):
                    stack.append(0)
                stack.append('+' if negstack[-1] else '-')
            else:
                if stack == [] or isinstance(stack[-1], str):
                    stack.append(0)
                stack[-1] = stack[-1] * 10 + int(s[i])

        ret = stack[0]

        for i in range(2, len(stack), 2):
            ret += stack[i] if stack[i - 1] == '+' else -stack[i]

        return ret
