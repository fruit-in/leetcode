class Solution:
    def countLargestGroup(self, n: int) -> int:
        counter = {}

        for i in range(1, n + 1):
            digits_sum = sum(map(int, str(i)))
            if digits_sum not in counter:
                counter[digits_sum] = 0
            counter[digits_sum] += 1

        largest_size = max(counter.values())

        return sum(map(lambda x: x == largest_size, counter.values()))
