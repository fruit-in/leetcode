import random


class RandomizedCollection:

    def __init__(self):
        self.numlist = []
        self.numindices = {}

    def insert(self, val: int) -> bool:
        self.numlist.append(val)
        if val not in self.numindices:
            self.numindices[val] = set()
        self.numindices[val].add(len(self.numlist) - 1)

        return len(self.numindices[val]) == 1

    def remove(self, val: int) -> bool:
        if val not in self.numindices:
            return False

        i = self.numindices[val].pop()
        if len(self.numindices[val]) == 0:
            self.numindices.pop(val)
        if i == len(self.numlist) - 1:
            self.numlist.pop()
        else:
            self.numlist[i] = self.numlist.pop()
            self.numindices[self.numlist[i]].remove(len(self.numlist))
            self.numindices[self.numlist[i]].add(i)

        return True

    def getRandom(self) -> int:
        return random.choice(self.numlist)


# Your RandomizedCollection object will be instantiated and called as such:
# obj = RandomizedCollection()
# param_1 = obj.insert(val)
# param_2 = obj.remove(val)
# param_3 = obj.getRandom()
