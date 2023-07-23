class Solution:
    def minimumSwap(self, s1: str, s2: str) -> int:
        xy, yx = 0, 0

        for i in range(len(s1)):
            if s1[i] == 'x' and s2[i] == 'y':
                xy += 1
            elif s1[i] == 'y' and s2[i] == 'x':
                yx += 1

        ret = xy // 2 + yx // 2

        if xy % 2 == yx % 2:
            return ret + xy % 2 * 2
        else:
            return -1
