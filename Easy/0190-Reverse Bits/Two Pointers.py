class Solution:
    # @param n, an integer
    # @return an integer
    def reverseBits(self, n):
        lo, hi = 1, 1 << 31
        for _ in range(16):
            if n & lo and not n & hi:
                n = (n | hi) & ~lo
            elif not n & lo and n & hi:
                n = (n | lo) & ~hi
            lo <<= 1
            hi >>= 1
        return n
