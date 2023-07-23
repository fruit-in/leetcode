class Solution:
    def findWinners(self, matches: List[List[int]]) -> List[List[int]]:
        count = {}
        answer = []

        for winner, loser in matches:
            if winner not in count:
                count[winner] = 0
            if loser not in count:
                count[loser] = 0
            count[loser] += 1

        answer.append(sorted(k for k, v in count.items() if v == 0))
        answer.append(sorted(k for k, v in count.items() if v == 1))

        return answer
