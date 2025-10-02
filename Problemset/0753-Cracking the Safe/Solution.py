class Solution:
    def crackSafe(self, n: int, k: int) -> str:
        if n == 1:
            return ''.join(map(str, range(k)))

        unused = {}
        stack = []
        curr = "0" * (n - 1)
        ret = []

        for num in range(10 ** (n - 1)):
            num = str(num).zfill(n - 1)

            if all(int(digit) < k for digit in num):
                unused[num] = [num[1:] + str(digit) for digit in range(k)]

        while curr != "":
            if unused[curr]:
                stack.append(curr)
                curr = unused[curr].pop()
            else:
                ret.append(curr[-1])
                curr = stack.pop() if stack else ""

        ret.append("0" * (n - 2))

        return ''.join(ret)
