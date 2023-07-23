# 859. 亲密字符串
给定两个由小写字母构成的字符串 ```A``` 和 ```B``` ，只要我们可以通过交换 ```A``` 中的两个字母得到与 ```B``` 相等的结果，就返回 ```true``` ；否则返回 ```false``` 。

#### 示例 1:
<pre>
<strong>输入:</strong> A = "ab", B = "ba"
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> A = "ab", B = "ab"
<strong>输出:</strong> false
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> A = "aa", B = "aa"
<strong>输出:</strong> true
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> A = "aaaaaaabc", B = "aaaaaaacb"
<strong>输出:</strong> true
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> A = "", B = "aa"
<strong>输出:</strong> false
</pre>

#### 提示:
1. ```0 <= A.length <= 20000```
2. ```0 <= B.length <= 20000```
3. ```A``` 和 ```B``` 仅由小写字母构成。

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def buddyStrings(self, A: str, B: str) -> bool:
        if len(A) != len(B):
            return False
        if A == B and len(set(A)) != len(A):
            return True
        a, b = '', ''
        for k, v in enumerate(A):
            if v != B[k]:
                a += v
                b += B[k]
            if len(a) > 2:
                return False
        return len(a) == 2 and a == b[::-1]
```
