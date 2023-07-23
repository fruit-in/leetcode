class Solution:
    def fizzBuzz(self, n: int) -> List[str]:
        dic = ((3, "Fizz"), (5, "Buzz"))
        ret = []

        for i in range(1, n + 1):
            op = ""

            for num, word in dic:
                if i % num == 0:
                    op += word

            if not op:
                op = str(i)

            ret.append(op)

        return ret
