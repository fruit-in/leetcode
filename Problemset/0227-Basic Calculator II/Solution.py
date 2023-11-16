class Solution:
    def calculate(self, s: str) -> int:
        isint = False
        lastsign = None
        stack = []
        ret = 0

        for ch in s:
            if ch in "+-*/":
                if lastsign == '*':
                    x, _ = stack.pop(), stack.pop()
                    stack[-1] *= x
                elif lastsign == '/':
                    x, _ = stack.pop(), stack.pop()
                    stack[-1] //= x

                isint = False
                lastsign = ch
                stack.append(ch)
            elif ch.isdigit():
                if isint:
                    stack[-1] = stack[-1] * 10 + int(ch)
                else:
                    isint = True
                    stack.append(int(ch))

        if lastsign == '*':
            x, _ = stack.pop(), stack.pop()
            stack[-1] *= x
        elif lastsign == '/':
            x, _ = stack.pop(), stack.pop()
            stack[-1] //= x

        lastsign = '+'

        for elem in stack:
            if isinstance(elem, int):
                if lastsign == '+':
                    ret += elem
                elif lastsign == '-':
                    ret -= elem
            else:
                lastsign = elem

        return ret
