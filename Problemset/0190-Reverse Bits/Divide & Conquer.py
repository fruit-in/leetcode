class Solution:
    # @param n, an integer
    # @return an integer
    def reverseBits(self, n):
        def reverse(n, lo, hi):
            if lo == hi:
                return n
            m = (lo + hi) / 2
            lo_val = n % (2 << m)
            hi_val = n - lo_val
            lo_rev = reverse(lo_val, lo , m)
            hi_rev = reverse(hi_val, m + 1, hi)
            return lo_rev * (1 << (hi - m)) + hi_rev / (1 << (hi - m))

        return reverse(n, 0, 31)
