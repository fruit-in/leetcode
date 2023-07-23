class Solution:
    def countTime(self, time: str) -> int:
        ret = 1

        if time[0] == '?' and time[1] == '?':
            ret *= 24
        elif time[0] == '?':
            ret *= 3 if time[1] < '4' else 2
        elif time[1] == '?':
            ret *= 10 if time[0] < '2' else 4
        ret *= 6 if time[3] == '?' else 1
        ret *= 10 if time[4] == '?' else 1

        return ret
