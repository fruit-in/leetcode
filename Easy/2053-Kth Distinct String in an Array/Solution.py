class Solution:
    def kthDistinct(self, arr: List[str], k: int) -> str:
        count = Counter(arr)
        distincts = [s for s in arr if count[s] == 1]

        return distincts[k - 1] if len(distincts) >= k else ""
