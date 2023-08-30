# 825. Friends Of Appropriate Ages
There are `n` persons on a social media website. You are given an integer array `ages` where `ages[i]` is the age of the <code>i<sup>th</sup></code> person.

A Person `x` will not send a friend request to a person `y` (`x != y`) if any of the following conditions is true:

* `age[y] <= 0.5 * age[x] + 7`
* `age[y] > age[x]`
* `age[y] > 100 && age[x] < 100`

Otherwise, `x` will send a friend request to `y`.

Note that if `x` sends a request to `y`, `y` will not necessarily send a request to `x`. Also, a person will not send a friend request to themself.

Return *the total number of friend requests made*.

#### Example 1:
<pre>
<strong>Input:</strong> ages = [16,16]
<strong>Output:</strong> 2
<strong>Explanation:</strong> 2 people friend request each other.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> ages = [16,17,18]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Friend requests are made 17 -> 16, 18 -> 17.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> ages = [20,30,100,110,120]
<strong>Output:</strong> 3
<strong>Explanation:</strong> Friend requests are made 110 -> 100, 120 -> 110, 120 -> 100.
</pre>

#### Constraints:
* `n == ages.length`
* <code>1 <= n <= 2 * 10<sup>4</sup></code>
* `1 <= ages[i] <= 120`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def numFriendRequests(self, ages: List[int]) -> int:
        n = len(ages)
        ret = 0

        ages.sort()

        for age in ages:
            if age > 14:
                ret += n - 1
                ret -= bisect.bisect_right(ages, 0.5 * age + 7)
                ret -= n - bisect.bisect_right(ages, age)

        return ret
```
