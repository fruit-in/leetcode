class Solution:
    def sortArrayByParityII(self, A: List[int]) -> List[int]:
        e, o = 0, 1
        result = [None] * len(A)
        for n in A:
            if n % 2 == 0:
                result[e] = n
                e += 2
            else:
                result[o] = n
                o += 2
        return result
