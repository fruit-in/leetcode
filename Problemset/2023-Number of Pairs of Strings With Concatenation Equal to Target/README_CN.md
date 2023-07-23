# 2023. 连接后等于目标字符串的字符串对
给你一个 **数字** 字符串数组 `nums` 和一个 **数字** 字符串 `target` ，请你返回 `nums[i] + nums[j]` （两个字符串连接）结果等于 `target` 的下标 `(i, j)` （需满足 `i != j`）的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = ["777","7","77","77"], target = "7777"
<strong>输出:</strong> 4
<strong>解释:</strong> 符合要求的下标对包括：
- (0, 1)："777" + "7"
- (1, 0)："7" + "777"
- (2, 3)："77" + "77"
- (3, 2)："77" + "77"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = ["123","4","12","34"], target = "1234"
<strong>输出:</strong> 2
<strong>解释:</strong> 符合要求的下标对包括
- (0, 1)："123" + "4"
- (2, 3)："12" + "34"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = ["1","1","1"], target = "11"
<strong>输出:</strong> 6
<strong>解释:</strong> 符合要求的下标对包括
- (0, 1)："1" + "1"
- (1, 0)："1" + "1"
- (0, 2)："1" + "1"
- (2, 0)："1" + "1"
- (1, 2)："1" + "1"
- (2, 1)："1" + "1"
</pre>

#### 提示:
* `2 <= nums.length <= 100`
* `1 <= nums[i].length <= 100`
* `2 <= target.length <= 100`
* `nums[i]` 和 `target` 只包含数字。
* `nums[i]` 和 `target` 不含有任何前导 0 。

## 题解 (Python)

### 1. 题解
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
