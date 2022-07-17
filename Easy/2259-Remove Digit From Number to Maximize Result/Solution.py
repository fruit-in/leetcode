class Solution:
    def removeDigit(self, number: str, digit: str) -> str:
        i = 0

        for j in range(len(number)):
            if number[j] == digit:
                i = j
                if j + 1 < len(number) and number[j] < number[j + 1]:
                    break

        return number[:i] + number[i + 1:]
