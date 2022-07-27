class Robot:

    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height
        self.pos = [0, 0]
        self.dir = "East"

    def step(self, num: int) -> None:
        num %= (self.width + self.height - 2) * 2

        if num == 0:
            num = (self.width + self.height - 2) * 2

        while num > 0:
            if self.dir == "East":
                if self.pos[0] + num < self.width:
                    self.pos[0] += num
                    num = 0
                else:
                    num -= self.width - 1 - self.pos[0]
                    self.pos[0] = self.width - 1
                    self.dir = "North"
            elif self.dir == "North":
                if self.pos[1] + num < self.height:
                    self.pos[1] += num
                    num = 0
                else:
                    num -= self.height - 1 - self.pos[1]
                    self.pos[1] = self.height - 1
                    self.dir = "West"
            elif self.dir == "West":
                if self.pos[0] - num >= 0:
                    self.pos[0] -= num
                    num = 0
                else:
                    num -= self.pos[0]
                    self.pos[0] = 0
                    self.dir = "South"
            else:
                if self.pos[1] - num >= 0:
                    self.pos[1] -= num
                    num = 0
                else:
                    num -= self.pos[1]
                    self.pos[1] = 0
                    self.dir = "East"

    def getPos(self) -> List[int]:
        return self.pos

    def getDir(self) -> str:
        return self.dir


# Your Robot object will be instantiated and called as such:
# obj = Robot(width, height)
# obj.step(num)
# param_2 = obj.getPos()
# param_3 = obj.getDir()
