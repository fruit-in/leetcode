class MajorityChecker:

    def __init__(self, arr: List[int]):
        self.arr = arr
        self.indices = {}

        for i, x in enumerate(arr):
            if x not in self.indices:
                self.indices[x] = []
            self.indices[x].append(i)

    def query(self, left: int, right: int, threshold: int) -> int:
        used = set()
        count = 0

        while right - left + 1 - count >= threshold:
            x = self.arr[randint(left, right)]
            while x in used:
                x = self.arr[randint(left, right)]
            used.add(x)
            indices = self.indices[x]
            occur = bisect_right(indices, right) - bisect_left(indices, left)
            count += occur
            if occur >= threshold:
                return x

        return -1


# Your MajorityChecker object will be instantiated and called as such:
# obj = MajorityChecker(arr)
# param_1 = obj.query(left,right,threshold)
