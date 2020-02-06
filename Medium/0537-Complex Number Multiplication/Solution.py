class Solution:
    def complexNumberMultiply(self, a: str, b: str) -> str:
        a = [int(x) for x in a[:-1].split('+')]
        b = [int(x) for x in b[:-1].split('+')]

        c = a[0] * b[0] - a[1] * b[1]
        d = a[0] * b[1] + a[1] * b[0]

        return "%d+%di" % (c, d)
