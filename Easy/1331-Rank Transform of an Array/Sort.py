class Solution:
    def arrayRankTransform(self, arr: List[int]) -> List[int]:
        sorted_arr = sorted(set(arr))
        rank = {n: i + 1 for i, n in enumerate(sorted_arr)}

        return [rank[n] for n in arr]
