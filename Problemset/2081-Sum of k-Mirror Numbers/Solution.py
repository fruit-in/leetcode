class Solution:
    def kMirror(self, k: int, n: int) -> int:
        x = 1
        nums = []

        while True:
            for a in range(x, 10 * x):
                b = c = int(str(a) + str(a)[-2::-1])
                d = ''
                while c > 0:
                    d = str(c % k) + d
                    c //= k
                if d == d[::-1]:
                    nums.append(b)
                if len(nums) == n:
                    return sum(nums)
            for a in range(x, 10 * x):
                b = c = int(str(a) + str(a)[::-1])
                d = ''
                while c > 0:
                    d = str(c % k) + d
                    c //= k
                if str(d) == str(d)[::-1]:
                    nums.append(b)
                if len(nums) == n:
                    return sum(nums)

            x *= 10
