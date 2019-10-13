# 412. Fizz Buzz
Write a program that outputs the string representation of numbers from 1 to *n*.

But for multiples of three it should output “Fizz” instead of the number and for the multiples of five output “Buzz”. For numbers which are multiples of both three and five output “FizzBuzz”.

#### Example:
```
n = 15,

Return:
[
    "1",
    "2",
    "Fizz",
    "4",
    "Buzz",
    "Fizz",
    "7",
    "8",
    "Fizz",
    "Buzz",
    "11",
    "Fizz",
    "13",
    "14",
    "FizzBuzz"
]
```

## Solutions (Python)

### 1. Simulation
```Python
class Solution:
    def fizzBuzz(self, n: int) -> List[str]:
        ret = []
        for i in range(1, n + 1):
            if i % 15 == 0:
                ret.append("FizzBuzz")
            elif i % 5 == 0:
                ret.append("Buzz")
            elif i % 3 == 0:
                ret.append("Fizz")
            else:
                ret.append(str(i))

        return ret
```

### 2. String Concatenation
```Python
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
```
