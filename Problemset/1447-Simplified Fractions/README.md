# 1447. Simplified Fractions
Given an integer `n`, return a list of all **simplified** fractions between 0 and 1 (exclusive) such that the denominator is less-than-or-equal-to `n`. The fractions can be in **any** order.

#### Example 1:
<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> ["1/2"]
<strong>Explanation:</strong> "1/2" is the only unique fraction with a denominator less-than-or-equal-to 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 3
<strong>Output:</strong> ["1/2","1/3","2/3"]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> ["1/2","1/3","1/4","2/3","3/4"]
<strong>Explanation:</strong> "2/4" is not a simplified fraction because it can be simplified to "1/2".
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> []
</pre>

#### Constraints:
* `1 <= n <= 100`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def simplifiedFractions(self, n: int) -> List[str]:
        return [
            "{}/{}".format(j, i)
            for i in range(2, n + 1)
            for j in range(1, i)
            if gcd(i, j) == 1
        ]
```

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer} n
# @return {String[]}
def simplified_fractions(n)
  ret = []

  (2..n).each do |i|
    (1...i).each do |j|
      ret.push(format('%d/%d', j, i)) if i.gcd(j) == 1
    end
  end

  ret
end
```
