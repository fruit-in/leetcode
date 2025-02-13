class Solution:
    def makesquare(self, matchsticks: List[int]) -> bool:
        perimeter = sum(matchsticks)

        if perimeter % 4 != 0:
            return False

        combs = {(0, 0, 0, 0)}

        for stick in matchsticks:
            newcombs = set()

            for comb in combs:
                for i in range(4):
                    if comb[i] + stick <= perimeter // 4:
                        newcomb = list(comb)
                        newcomb[i] += stick
                        newcombs.add(tuple(sorted(newcomb)))

            combs = newcombs

        return len(combs) > 0
