class Solution:
    def longestSubsequenceRepeatedK(self, s: str, k: int) -> str:
        def dfs() -> None:
            for ch in count:
                if count[ch] > 0:
                    seq.append(ch)
                    count[ch] -= 1
                    seqs.append(''.join(seq))
                    dfs()
                    seq.pop()
                    count[ch] += 1

        count = {ch: cnt // k for ch,
                 cnt in collections.Counter(s).items() if cnt >= k}
        seq = []
        seqs = []

        dfs()
        seqs.sort(key=lambda seq: (len(seq), seq), reverse=True)

        for seq in seqs:
            i = 0
            j = 0

            for c in s:
                if seq[i] == c:
                    i += 1
                if i == len(seq):
                    i = 0
                    j += 1
                if j == k:
                    return seq

        return ""
