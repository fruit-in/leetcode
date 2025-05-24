class Solution:
    def shoppingOffers(self, price: List[int], special: List[List[int]], needs: List[int]) -> int:
        @cache
        def dfs(needs: Tuple[int, ...]) -> int:
            needs = list(needs)
            ret = sum(needs[i] * price[i] for i in range(len(needs)))

            for offer in special:
                newneeds = tuple(needs[i] - offer[i]
                                 for i in range(len(needs)))
                if all(x >= 0 for x in newneeds):
                    ret = min(ret, offer[-1] + dfs(newneeds))

            return ret

        special = [offer for offer in special if sum(
            offer[i] * price[i] for i in range(len(price))) > offer[-1]]

        return dfs(tuple(needs))
