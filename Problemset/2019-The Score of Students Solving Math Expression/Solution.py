class Solution:
    def scoreOfStudents(self, s: str, answers: List[int]) -> int:
        @cache
        def subExpressionAnswers(l: int, r: int) -> Set[int]:
            if l == r:
                return {int(s[l])}

            ret = set()

            for i in range(l + 1, r, 2):
                for x in subExpressionAnswers(l, i - 1):
                    for y in subExpressionAnswers(i + 1, r):
                        if s[i] == '+':
                            ret.add(min(x + y, maxanswer + 1))
                        else:
                            ret.add(min(x * y, maxanswer + 1))

            return ret

        maxanswer = max(answers)
        correctanswer = eval(s)
        wronganswers = subExpressionAnswers(0, len(s) - 1) - {correctanswer}
        correctpoints = sum(5 for a in answers if a == correctanswer)
        wrongpoints = sum(2 for a in answers if a in wronganswers)

        return correctpoints + wrongpoints
