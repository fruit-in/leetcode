# 1980. 找出不同的二进制字符串
给你一个字符串数组 `nums` ，该数组由 `n` 个 **互不相同** 的二进制字符串组成，且每个字符串长度都是 `n` 。请你找出并返回一个长度为 `n` 且 **没有出现** 在 `nums` 中的二进制字符串。如果存在多种答案，只需返回 **任意一个** 即可。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = ["01","10"]
<strong>输出:</strong> "11"
<strong>解释:</strong> "11" 没有出现在 nums 中。"00" 也是正确答案。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = ["00","01"]
<strong>输出:</strong> "11"
<strong>解释:</strong> "11" 没有出现在 nums 中。"10" 也是正确答案。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = ["111","011","001"]
<strong>输出:</strong> "101"
<strong>解释:</strong> "101" 没有出现在 nums 中。"000"、"010"、"100"、"110" 也是正确答案。
</pre>

#### 提示:
* `n == nums.length`
* `1 <= n <= 16`
* `nums[i].length == n`
* `nums[i]` 为 `'0'` 或 `'1'`
* `nums` 中的所有字符串 **互不相同**

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def findDifferentBinaryString(self, nums: List[str]) -> str:
        nums = {int(num, 2) for num in nums}

        for x in range(2 ** len(nums)):
            if x not in nums:
                return bin(x)[2:].zfill(len(nums))
```
