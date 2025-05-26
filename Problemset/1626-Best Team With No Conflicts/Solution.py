class Solution:
    def bestTeamScore(self, scores: List[int], ages: List[int]) -> int:
        players = sorted(zip(ages, scores))
        sl = SortedList([(0, 0)])

        for age, score in players:
            i = sl.bisect_right((score, inf))
            sl.add((score, sl[i - 1][1] + score))
            while i + 1 < len(sl) and sl[i + 1][1] <= sl[i][1]:
                sl.pop(i + 1)
            if sl[i - 1][0] == score:
                sl.pop(i - 1)

        return sl[-1][1]
