class Solution:
    def minimizeResult(self, expression: str) -> str:
        m = expression.find("+")
        smallest = float("inf")
        ret = ""

        for l in range(m):
            for r in range(m + 2, len(expression) + 1):
                x = int(expression[l:m]) + int(expression[m + 1:r])
                if l > 0:
                    x *= int(expression[:l])
                if r < len(expression):
                    x *= int(expression[r:])

                if x < smallest:
                    smallest = x
                    ret = expression[:l] + \
                        "(" + expression[l:r] + ")" + expression[r:]

        return ret
