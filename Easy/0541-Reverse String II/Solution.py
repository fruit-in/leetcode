class Solution:
    def reverseStr(self, s: str, k: int) -> str:
        s_list = list(s)

        for i in range(0, len(s), 2 * k):
            j = min(i + k - 1, len(s) - 1)
            while i < j:
                s_list[i], s_list[j] = s_list[j], s_list[i]
                i += 1
                j -= 1

        return ''.join(s_list)
