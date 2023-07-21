class Solution:
    def topStudents(self, positive_feedback: List[str], negative_feedback: List[str], report: List[str], student_id: List[int], k: int) -> List[int]:
        positive_feedback = set(positive_feedback)
        negative_feedback = set(negative_feedback)
        students = {}

        for (words, student) in zip(report, student_id):
            students[student] = 0

            for word in words.split():
                if word in positive_feedback:
                    students[student] += 3
                elif word in negative_feedback:
                    students[student] -= 1

        return sorted(student_id, key=lambda x: (-students[x], x))[:k]
