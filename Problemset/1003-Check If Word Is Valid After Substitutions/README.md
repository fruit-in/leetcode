# 1003. Check If Word Is Valid After Substitutions
We are given that the string ```"abc"``` is valid.

From any valid string ```V```, we may split ```V``` into two pieces ```X``` and ```Y``` such that ```X + Y``` (```X``` concatenated with ```Y```) is equal to ```V```.  (```X``` or ```Y``` may be empty.)  Then, ```X + "abc" + Y``` is also valid.

If for example ```S = "abc"```, then examples of valid strings are: ```"abc", "aabcbc", "abcabc", "abcabcababcc"```.  Examples of **invalid** strings are: ```"abccba"```, ```"ab"```, ```"cababc"```, ```"bac"```.

Return ```true``` if and only if the given string ```S``` is valid.

#### Example 1:
<pre>
<strong>Input:</strong> "aabcbc"
<strong>Output:</strong> true
<strong>Explanation:</strong>
We start with the valid string "abc".
Then we can insert another "abc" between "a" and "bc", resulting in "a" + "abc" + "bc" which is "aabcbc".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "abcabcababcc"
<strong>Output:</strong> true
<strong>Explanation:</strong>
"abcabcabc" is valid after consecutive insertings of "abc".
Then we can insert "abc" before the last letter, resulting in "abcabcab" + "abc" + "c" which is "abcabcababcc".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> "abccba"
<strong>Output:</strong> false
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> "cababc"
<strong>Output:</strong> false
</pre>

#### Note:
1. ```1 <= S.length <= 20000```
2. ```S[i]``` is ```'a'```, ```'b'```, or ```'c'```

## Solutions (Python)

### 1. Replace
```Python
class Solution:
    def isValid(self, S: str) -> bool:
        while 'abc' in S:
            S = S.replace('abc', '')

        return not S
```
