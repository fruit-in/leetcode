class OrderedStream:

    def __init__(self, n: int):
        self.values = [None] * n
        self.ptr = 0

    def insert(self, id: int, value: str) -> List[str]:
        self.values[id - 1] = value
        start = self.ptr

        while self.ptr < len(self.values) and self.values[self.ptr] is not None:
            self.ptr += 1

        return self.values[start:self.ptr]


# Your OrderedStream object will be instantiated and called as such:
# obj = OrderedStream(n)
# param_1 = obj.insert(id,value)
