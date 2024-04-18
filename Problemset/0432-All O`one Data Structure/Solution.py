class AllOne:

    def __init__(self):
        self.keys = []
        self.count = {}
        self.index = {}
        self.range = {}

    def inc(self, key: str) -> None:
        if key not in self.count:
            self.keys.append(key)
            self.count[key] = 1
            self.index[key] = len(self.keys) - 1
            if 1 not in self.range:
                self.range[1] = [self.index[key], self.index[key]]
            self.range[1][1] = self.index[key]
        else:
            count0 = self.count[key]
            count1 = count0 + 1
            i = self.range[count0][0]
            j = self.index[key]
            self.index[self.keys[i]], self.index[self.keys[j]] = j, i
            self.keys[i], self.keys[j] = self.keys[j], self.keys[i]
            self.count[key] += 1
            self.range[count0][0] += 1
            if self.range[count0][0] > self.range[count0][1]:
                self.range.pop(count0)
            if count1 not in self.range:
                self.range[count1] = [i, i]
            self.range[count1][1] = i

    def dec(self, key: str) -> None:
        if self.count[key] == 1:
            self.index[self.keys[-1]] = self.index[key]
            self.keys[self.index[key]] = self.keys[-1]
            self.keys.pop()
            self.count.pop(key)
            self.index.pop(key)
            self.range[1][1] -= 1
            if self.range[1][0] > self.range[1][1]:
                self.range.pop(1)
        else:
            count0 = self.count[key]
            count1 = count0 - 1
            i = self.range[count0][1]
            j = self.index[key]
            self.index[self.keys[i]], self.index[self.keys[j]] = j, i
            self.keys[i], self.keys[j] = self.keys[j], self.keys[i]
            self.count[key] -= 1
            self.range[count0][1] -= 1
            if self.range[count0][0] > self.range[count0][1]:
                self.range.pop(count0)
            if count1 not in self.range:
                self.range[count1] = [i, i]
            self.range[count1][0] = i

    def getMaxKey(self) -> str:
        return self.keys[0] if self.keys != [] else ""

    def getMinKey(self) -> str:
        return self.keys[-1] if self.keys != [] else ""


# Your AllOne object will be instantiated and called as such:
# obj = AllOne()
# obj.inc(key)
# obj.dec(key)
# param_3 = obj.getMaxKey()
# param_4 = obj.getMinKey()
