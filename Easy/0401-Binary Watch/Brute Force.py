class Solution:
    def readBinaryWatch(self, num: int) -> List[str]:
        hours = [[], [], [], []]
        mins = [[], [], [], [], [], []]
        time = []

        for n in range(0, 60):
            if n < 12:
                hours[bin(n).count('1')].append(n)
            mins[bin(n).count('1')].append(n)

        for i in range(0, num + 1):
            if i < 4 and num - i < 6:
                time.extend("%d:%02d" % (h, m) for h in hours[i] for m in mins[num - i])

        return time
