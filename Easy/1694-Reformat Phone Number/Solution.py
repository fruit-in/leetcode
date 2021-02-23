class Solution:
    def reformatNumber(self, number: str) -> str:
        digits = number.replace("-", "").replace(" ", "")
        blocks = [digits[i:i + 3] for i in range(0, len(digits), 3)]

        if len(blocks[-1]) == 1:
            blocks[-1] = blocks[-2][2] + blocks[-1]
            blocks[-2] = blocks[-2][:2]

        return "-".join(blocks)
