class Solution:
    def largestTimeFromDigits(self, A: List[int]) -> str:
        cnt0_3 = sum(1 if x < 4 else 0 for x in A)
        cnt0_5 = sum(1 if x < 6 else 0 for x in A)
        if 2 in A and cnt0_3 > 1 and cnt0_5 > 2:
            A.remove(2)
            hour = "2" + str(max(filter(lambda x: x < 4, A)))
            A.remove(max(filter(lambda x: x < 4, A)))
        elif 1 in A:
            A.remove(1)
            hour = "1" + str(max(A))
            A.remove(max(A))
        elif 0 in A:
            A.remove(0)
            hour = "0" + str(max(A))
            A.remove(max(A))
        else:
            return ""

        if max(A) < 6:
            return hour + ":" + str(max(A)) + str(min(A))
        elif min(A) < 6:
            return hour + ":" + str(min(A)) + str(max(A))
        else:
            return ""
