class Bitset:

    def __init__(self, size: int):
        self.bitset = [False] * size
        self.count1 = 0
        self.flipflag = False

    def fix(self, idx: int) -> None:
        if self.bitset[idx] == self.flipflag:
            self.bitset[idx] = not self.flipflag
            self.count1 += 1

    def unfix(self, idx: int) -> None:
        if self.bitset[idx] != self.flipflag:
            self.bitset[idx] = self.flipflag
            self.count1 -= 1

    def flip(self) -> None:
        self.count1 = len(self.bitset) - self.count1
        self.flipflag = not self.flipflag

    def all(self) -> bool:
        return self.count1 == len(self.bitset)

    def one(self) -> bool:
        return self.count1 > 0

    def count(self) -> int:
        return self.count1

    def toString(self) -> str:
        return ''.join('1' if b ^ self.flipflag else '0' for b in self.bitset)


# Your Bitset object will be instantiated and called as such:
# obj = Bitset(size)
# obj.fix(idx)
# obj.unfix(idx)
# obj.flip()
# param_4 = obj.all()
# param_5 = obj.one()
# param_6 = obj.count()
# param_7 = obj.toString()
