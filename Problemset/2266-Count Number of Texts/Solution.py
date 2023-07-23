class Solution:
    def countTexts(self, pressedKeys: str) -> int:
        digit = '0'
        count = 0
        max3 = [1, 1, 2]
        max4 = [1, 1, 2, 4]
        ret = 1

        for d in (pressedKeys + '0'):
            if d == digit:
                count += 1
            else:
                if digit in "79":
                    while count >= len(max4):
                        max4.append(sum(max4[-4:]))
                    ret *= max4[count]
                else:
                    while count >= len(max3):
                        max3.append(sum(max3[-3:]))
                    ret *= max3[count]

                digit = d
                count = 1

        return ret % 1_000_000_007
