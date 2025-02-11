from itertools import permutations


class Solution:
    def maxCompatibilitySum(self, students: List[List[int]], mentors: List[List[int]]) -> int:
        ret = 0

        for perm in permutations(students):
            score = 0

            for student, mentor in zip(perm, mentors):
                for i in range(len(student)):
                    if student[i] == mentor[i]:
                        score += 1

            ret = max(ret, score)

        return ret
