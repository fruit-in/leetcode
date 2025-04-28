class Solution:
    def abbreviateProduct(self, left: int, right: int) -> str:
        pre = 0
        suf = 1
        ec = 0
        ret = ""

        for x in range(left, right + 1):
            pre *= x
            suf *= x

            while suf % 10 == 0:
                suf //= 10
                ec += 1

            if pre == 0 and suf > 9999999999999999999999999999:
                pre = int(str(suf)[:17])
                suf %= 100000000000
            elif pre > 0 and suf > 99999999999:
                suf %= 100000000000

            while pre > 99999999999999999:
                pre //= 10

        if pre == 0 and suf > 9999999999:
            pre = int(str(suf)[:5])
            suf = int(str(suf)[-5:])

        if pre == 0:
            return str(suf) + "e" + str(ec)
        else:
            return str(pre)[:5] + "..." + "{:05}".format(suf)[-5:] + "e" + str(ec)
