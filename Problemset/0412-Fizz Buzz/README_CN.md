# 412. Fizz Buzz
写一个程序，输出从 1 到 *n* 数字的字符串表示。
1. 如果 *n* 是3的倍数，输出“Fizz”；
2. 如果 *n* 是5的倍数，输出“Buzz”；
3. 如果 *n* 同时是3和5的倍数，输出 “FizzBuzz”。

#### 示例:
```
n = 15,

返回:
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

## 题解 (Python)

### 1. 模拟
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

### 2. 字符串连接
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
