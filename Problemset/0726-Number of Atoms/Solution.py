class Solution:
    def countOfAtoms(self, formula: str) -> str:
        stack = []
        factors = []
        x = 1
        counter = {}
        ret = []

        for i, c in enumerate(formula):
            if c.islower():
                stack[-1][0] += c
            elif c.isdigit():
                if not formula[i - 1].isdigit():
                    stack[-1][1] = 0
                stack[-1][1] = stack[-1][1] * 10 + int(c)
            else:
                stack.append([c, 1])

        while stack:
            atom, count = stack.pop()

            if atom == ')':
                factors.append(count)
                x *= count
            elif atom == '(':
                x //= factors.pop()
            else:
                if atom not in counter:
                    counter[atom] = 0
                counter[atom] += count * x

        for atom, count in sorted(counter.items()):
            if count > 1:
                ret.append(atom + str(count))
            else:
                ret.append(atom)

        return ''.join(ret)
