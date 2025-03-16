class Fancy:
    MOD = 1000000007

    def __init__(self):
        self.vals = []
        self.inc = 0
        self.m = 1

    def append(self, val: int) -> None:
        self.vals.append((val - self.inc, self.m))

    def addAll(self, inc: int) -> None:
        self.inc = (self.inc + inc) % self.MOD

    def multAll(self, m: int) -> None:
        self.inc = self.inc * m % self.MOD
        self.m = self.m * m % self.MOD

    def getIndex(self, idx: int) -> int:
        if idx >= len(self.vals):
            return -1

        return (self.vals[idx][0] * pow(self.vals[idx][1], self.MOD - 2, self.MOD) * self.m + self.inc) % self.MOD


# Your Fancy object will be instantiated and called as such:
# obj = Fancy()
# obj.append(val)
# obj.addAll(inc)
# obj.multAll(m)
# param_4 = obj.getIndex(idx)
