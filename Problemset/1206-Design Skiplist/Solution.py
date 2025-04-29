class Skiplist:

    def __init__(self, val=-1, isroot=True):
        self.val = val
        self.isroot = isroot
        self.next = None
        self.nextlevel = None

    def search(self, target: int) -> bool:
        if self.val == target and not self.isroot:
            return True
        if self.val > target and not self.isroot:
            return False

        if self.next is not None and self.next.val <= target:
            return self.next.search(target)
        else:
            return self.nextlevel is not None and self.nextlevel.search(target)

    def add(self, num: int) -> None:
        if self.nextlevel is None:
            self.next = Skiplist(num, False)
            self.next.nextlevel = Skiplist(num, False)
            self.nextlevel = Skiplist(self.val, self.isroot)
        elif self.next is None or self.next.val >= num:
            self.nextlevel.add(num)
        else:
            self.next.add(num)

    def erase(self, num: int) -> bool:
        if self.next is not None:
            if self.next.val < num:
                return self.next.erase(num)
            elif self.next.val == num:
                stack = []
                curr = self.next
                while curr is not None:
                    stack.append(curr)
                    curr = curr.nextlevel

                if len(stack) > 2:
                    stack.pop()
                    stack[-1].nextlevel = stack.pop().next.nextlevel
                    for i in range(len(stack) - 1, -1, -1):
                        stack[i].val = stack[i].nextlevel.val
                else:
                    self.next = self.nextlevel.next
                    self.nextlevel = self.nextlevel.nextlevel

                return True

        return self.nextlevel is not None and self.nextlevel.erase(num)


# Your Skiplist object will be instantiated and called as such:
# obj = Skiplist()
# param_1 = obj.search(target)
# obj.add(num)
# param_3 = obj.erase(num)
