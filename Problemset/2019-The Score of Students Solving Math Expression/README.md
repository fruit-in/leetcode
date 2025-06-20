# 2019. The Score of Students Solving Math Expression
You are given a string `s` that contains digits `0-9`, addition symbols `'+'`, and multiplication symbols `'*'` **only**, representing a **valid** math expression of **single digit numbers** (e.g., `3+5*2`). This expression was given to `n` elementary school students. The students were instructed to get the answer of the expression by following this **order of operations**:
* Compute **multiplication**, reading from **left to right**; Then,
* Compute **addition**, reading from **left to right**.

You are given an integer array `answers` of length `n`, which are the submitted answers of the students in no particular order. You are asked to grade the `answers`, by following these **rules**:
* If an answer **equals** the correct answer of the expression, this student will be rewarded `5` points;
* Otherwise, if the answer **could be interpreted** as if the student applied the operators **in the wrong order** but had **correct arithmetic**, this student will be rewarded `2` points;
* Otherwise, this student will be rewarded `0` points.

Return *the sum of the points of the students*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/09/17/student_solving_math.png)
<pre>
<strong>Input:</strong> s = "7+3*1*2", answers = [20,13,42]
<strong>Output:</strong> 7
<strong>Explanation:</strong> As illustrated above, the correct answer of the expression is 13, therefore one student is rewarded 5 points: [20,13,42]
A student might have applied the operators in this wrong order: ((7+3)*1)*2 = 20. Therefore one student is rewarded 2 points: [20,13,42]
The points for the students are: [2,5,0]. The sum of the points is 2+5+0=7.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "3+5*2", answers = [13,0,10,13,13,16,16]
<strong>Output:</strong> 19
<strong>Explanation:</strong> The correct answer of the expression is 13, therefore three students are rewarded 5 points each: [13,0,10,13,13,16,16]
A student might have applied the operators in this wrong order: ((3+5)*2 = 16. Therefore two students are rewarded 2 points: [13,0,10,13,13,16,16]
The points for the students are: [5,0,0,5,5,2,2]. The sum of the points is 5+0+0+5+5+2+2=19.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "6+0*1", answers = [12,9,6,4,8,6]
<strong>Output:</strong> 10
<strong>Explanation:</strong> The correct answer of the expression is 6.
If a student had incorrectly done (6+0)*1, the answer would also be 6.
By the rules of grading, the students will still be rewarded 5 points (as they got the correct answer), not 2 points.
The points for the students are: [0,0,5,0,0,5]. The sum of the points is 10.
</pre>

#### Constraints:
* `3 <= s.length <= 31`
* `s` represents a valid expression that contains only digits `0-9`, `'+'`, and `'*'` only.
* All the integer operands in the expression are in the **inclusive** range `[0, 9]`.
* `1 <=` The count of all operators (`'+'` and `'*'`) in the math expression `<= 15`
* Test data are generated such that the correct answer of the expression is in the range of `[0, 1000]`.
* `n == answers.length`
* <code>1 <= n <= 10<sup>4</sup></code>
* `0 <= answers[i] <= 1000`

## Solutions (Python)

### 1. Solution
```Python
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
```
