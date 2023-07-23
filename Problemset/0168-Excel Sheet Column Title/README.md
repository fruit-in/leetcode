# 168. Excel Sheet Column Title
Given a positive integer, return its corresponding column title as appear in an Excel sheet.

For example:
```
    1 -> A
    2 -> B
    3 -> C
    ...
    26 -> Z
    27 -> AA
    28 -> AB
    ...
```

#### Example 1:
<pre>
<strong>Input:</strong> 1
<strong>Output:</strong> "A"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 28
<strong>Output:</strong> "AB"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> 701
<strong>Output:</strong> "ZY"
</pre>

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def convertToTitle(self, n: int) -> str:
        ret = ""
        while n > 0:
            n -= 1
            ret = chr(n % 26 + 65) + ret
            n //= 26
        return ret
```

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer} n
# @return {String}
def convert_to_title(n)
    ret = ""

    while n > 0
        n -= 1
        ret = (n % 26 + 65).chr + ret
        n /= 26
    end

    return ret
end
```
