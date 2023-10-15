class Solution:
    def nearestPalindromic(self, n: str) -> str:
        nums = []
        nums.append(str(int(n[:(len(n) + 1) // 2])))
        nums.append(str(int(n[:(len(n) + 1) // 2]) - 1))
        nums.append(str(int(n[:(len(n) + 1) // 2]) + 1))
        nums[0] += nums[0][:len(n) // 2][::-1]
        nums[1] += nums[1][:len(n) // 2][::-1]
        nums[2] += nums[2][:len(n) // 2][::-1]
        nums.append("9" * max(len(n) - 1, 1))
        nums.sort(key=lambda x: (x == n, abs(int(x) - int(n)), int(x)))

        return nums[0]
