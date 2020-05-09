class Solution:
    def maxProduct(self, words: List[str]) -> int:
        mask_len = {}
        ret = 0

        for word in words:
            mask = 0
            for ch in word:
                mask |= 1 << (ord(ch) - 97)
            if mask not in mask_len:
                mask_len[mask] = 0
            mask_len[mask] = max(mask_len[mask], len(word))

        mask_len = list(mask_len.items())
        for i in range(len(mask_len)):
            for j in range(i + 1, len(mask_len)):
                if mask_len[i][0] & mask_len[j][0] == 0:
                    ret = max(ret, mask_len[i][1] * mask_len[j][1])

        return ret
