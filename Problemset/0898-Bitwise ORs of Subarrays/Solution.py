class Solution:
    def subarrayBitwiseORs(self, arr: List[int]) -> int:
        subarrayors = set(arr)

        for i in range(len(arr)):
            for j in range(i - 1, -1, -1):
                if arr[j] | arr[i] == arr[j]:
                    break
                arr[j] |= arr[i]
                subarrayors.add(arr[j])

        return len(subarrayors)
