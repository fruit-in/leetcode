class Solution:
    def convertTime(self, current: str, correct: str) -> int:
        current = int(current[:2]) * 60 + int(current[3:])
        correct = int(correct[:2]) * 60 + int(correct[3:])
        diff = correct - current + (1440 if correct < current else 0)
        ret = 0

        ret, diff = ret + diff // 60,  diff % 60
        ret, diff = ret + diff // 15,  diff % 15
        ret, diff = ret + diff // 5,  diff % 5
        ret += diff

        return ret
