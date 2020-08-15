# 38. Count and Say
The count-and-say sequence is the sequence of integers with the first five terms as following:

```
1.     1
2.     11
3.     21
4.     1211
5.     111221
```

```1``` is read off as ```"one 1"``` or ```11```.

```11``` is read off as ```"two 1s"``` or ```21```.

```21``` is read off as ```"one 2```, then ```one 1"``` or ```1211```.

Given an integer *n* where 1 ≤ *n* ≤ 30, generate the *n*<sup>th</sup> term of the count-and-say sequence.

Note: Each term of the sequence of integers will be represented as a string.

#### Example 1:
<pre>
<strong>Input:</strong> 1
<strong>Output:</strong> "1"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 4
<strong>Output:</strong> "1211"
</pre>

## Solutions (Python)

### 1. Two Pointers
```Python3
class Solution:
    def countAndSay(self, n: int) -> str:
        s = "1"
        for i in range(n - 1):
            tmp = ""
            i = 0
            for j in range(len(s)):
                if s[i] != s[j]:
                    tmp += str(j - i) + s[i]
                    i = j
            tmp += str(len(s) - i) + s[i]
            s = tmp
        return s
```

## Solutions (Ruby)

### 1. Two Pointers
```Ruby
# @param {Integer} n
# @return {String}
def count_and_say(n)
    s = "1"

    for _ in 1...n
        tmp = ""
        i = 0

        for j in 0...s.length
            if s[i] != s[j]
                tmp += (j - i).to_s + s[i]
                i = j
            end
        end

        tmp += (s.length - i).to_s + s[i]
        s = tmp
    end

    return s
end
```
