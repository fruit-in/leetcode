# 2023. Number of Pairs of Strings With Concatenation Equal to Target
Given an array of **digit** strings `nums` and a **digit** string `target`, return *the number of pairs of indices* `(i, j)` (*where* `i != j`) *such that the **concatenation** of* `nums[i] + nums[j]` *equals* `target`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = ["777","7","77","77"], target = "7777"
<strong>Output:</strong> 4
<strong>Explanation:</strong> Valid pairs are:
- (0, 1): "777" + "7"
- (1, 0): "7" + "777"
- (2, 3): "77" + "77"
- (3, 2): "77" + "77"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = ["123","4","12","34"], target = "1234"
<strong>Output:</strong> 2
<strong>Explanation:</strong> Valid pairs are:
- (0, 1): "123" + "4"
- (2, 3): "12" + "34"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = ["1","1","1"], target = "11"
<strong>Output:</strong> 6
<strong>Explanation:</strong> Valid pairs are:
- (0, 1): "1" + "1"
- (1, 0): "1" + "1"
- (0, 2): "1" + "1"
- (2, 0): "1" + "1"
- (1, 2): "1" + "1"
- (2, 1): "1" + "1"
</pre>

#### Constraints:
* `2 <= nums.length <= 100`
* `1 <= nums[i].length <= 100`
* `2 <= target.length <= 100`
* `nums[i]` and `target` consist of digits.
* `nums[i]` and `target` do not have leading zeros.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def numOfPairs(self, nums: List[str], target: str) -> int:
        count = {}
        ret = 0

        for num in nums:
            if target.startswith(num) and target[len(num):] in count:
                ret += count[target[len(num):]]
            if target.endswith(num) and target[:-len(num)] in count:
                ret += count[target[:-len(num)]]

            if num not in count:
                count[num] = 0
            count[num] += 1

        return ret
```
