# 1523. Count Odd Numbers in an Interval Range
Given two non-negative integers `low` and `high`. Return the *count of odd numbers between* `low` *and* `high` *(inclusive)*.

#### Example 1:
<pre>
<strong>Input:</strong> low = 3, high = 7
<strong>Output:</strong> 3
<strong>Explanation:</strong> The odd numbers between 3 and 7 are [3,5,7].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> low = 8, high = 10
<strong>Output:</strong> 1
<strong>Explanation:</strong> The odd numbers between 8 and 10 are [9].
</pre>

#### Constraints:
* `0 <= low <= high <= 10^9`

## Solutions (Ruby)

### 1. Mathematical
```Ruby
# @param {Integer} low
# @param {Integer} high
# @return {Integer}
def count_odds(low, high)
    return (high - low + low % 2 + high % 2) / 2
end
```
