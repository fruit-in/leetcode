class Solution:
    def minMaxDifference(self, num: int) -> int:
        num = str(num)
        maxnum = int(num)
        minnum = int(num.replace(num[0], '0'))

        for i in range(len(num)):
            if num[i] != '9':
                maxnum = int(num.replace(num[i], '9'))
                break

        return maxnum - minnum
