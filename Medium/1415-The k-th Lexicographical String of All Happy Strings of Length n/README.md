# 1415. The k-th Lexicographical String of All Happy Strings of Length n
A **happy string** is a string that:
* consists only of letters of the set `['a', 'b', 'c']`.
* `s[i] != s[i + 1]` for all values of `i` from `1` to `s.length - 1` (string is 1-indexed).

For example, strings **"abc"**, **"ac"**, **"b"** and **"abcbabcbcb"** are all happy strings and strings **"aa"**, **"baa"** and **"ababbc"** are not happy strings.

Given two integers `n` and `k`, consider a list of all happy strings of length `n` sorted in lexicographical order.

Return *the kth string* of this list or return an **empty string** if there are less than `k` happy strings of length `n`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 1, k = 3
<strong>Output:</strong> "c"
<strong>Explanation:</strong> The list ["a", "b", "c"] contains all happy strings of length 1. The third string is "c".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 1, k = 4
<strong>Output:</strong> ""
<strong>Explanation:</strong> There are only 3 happy strings of length 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 3, k = 9
<strong>Output:</strong> "cab"
<strong>Explanation:</strong> There are 12 different happy string of length 3 ["aba", "abc", "aca", "acb", "bab", "bac", "bca", "bcb", "cab", "cac", "cba", "cbc"]. You will find the 9th string = "cab"
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> n = 2, k = 7
<strong>Output:</strong> ""
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> n = 10, k = 100
<strong>Output:</strong> "abacbabacb"
</pre>

#### Constraints:
* `1 <= n <= 10`
* `1 <= k <= 100`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def getHappyString(self, n: int, k: int) -> str:
        happy = list("abc")

        for _ in range(n - 1):
            happy_ = []

            for s in happy:
                for c in "abc":
                    if s[-1] != c:
                        happy_.append(s + c)

            happy = happy_

        return "" if k > len(happy) else happy[k - 1]
```

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer} n
# @param {Integer} k
# @return {String}
def get_happy_string(n, k)
  happy = %w[a b c]

  (2..n).each do |_i|
    happy_ = []

    happy.each do |s|
      'abc'.chars.each do |c|
        happy_.push(s + c) if s[-1] != c
      end
    end

    happy = happy_
  end

  k > happy.size ? '' : happy[k - 1]
end
```
