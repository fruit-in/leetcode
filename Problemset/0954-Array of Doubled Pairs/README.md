# 954. Array of Doubled Pairs
Given an integer array of even length `arr`, return `true` *if it is possible to reorder* `arr` *such that* `arr[2 * i + 1] = 2 * arr[2 * i]` *for every* `0 <= i < len(arr) / 2`, *or* `false` *otherwise*.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [3,1,3,6]
<strong>Output:</strong> false
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [2,1,2,6]
<strong>Output:</strong> false
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [4,-2,2,-4]
<strong>Output:</strong> true
<strong>Explanation:</strong> We can take two groups, [-2,-4] and [2,4] to form [-2,-4,2,4] or [2,4,-2,-4].
</pre>

#### Constraints:
* <code>2 <= arr.length <= 3 * 10<sup>4</sup></code>
* `arr.length` is even.
* <code>-10<sup>5</sup> <= arr[i] <= 10<sup>5</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def canReorderDoubled(self, arr: List[int]) -> bool:
        count = {}
        nums = []

        for x in arr:
            if x % 2 == 0:
                if x not in count:
                    count[x] = 0
                count[x] += 1
            else:
                if 2 * x not in count:
                    count[2 * x] = 0
                count[2 * x] -= 1

        for x, v in count.items():
            if v < 0:
                return False
            elif v > 0:
                nums.append(x)

        nums.sort(key=lambda x: (x >= 0, abs(x)))

        for x in nums:
            if count[x] == 0:
                continue
            elif 2 * x in count and count[x] <= count[2 * x]:
                count[2 * x] -= count[x]
                count[x] = 0
            else:
                return False

        return True
```
