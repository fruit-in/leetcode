class Solution:
    def timeRequiredToBuy(self, tickets: List[int], k: int) -> int:
        ret = 0

        for i in range(len(tickets)):
            if i <= k:
                ret += min(tickets[i], tickets[k])
            else:
                ret += min(tickets[i], tickets[k] - 1)

        return ret
