# 2442. 反转之后不同整数的数目
给你一个由 **正** 整数组成的数组 `nums` 。

你必须取出数组中的每个整数，**反转其中每个数位**，并将反转后得到的数字添加到数组的末尾。这一操作只针对 `nums` 中原有的整数执行。

返回结果数组中 **不同** 整数的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,13,10,12,31]
<strong>输出:</strong> 6
<strong>解释:</strong> 反转每个数字后，结果数组是 [1,13,10,12,31,1,31,1,21,13] 。
反转后得到的数字添加到数组的末尾并按斜体加粗表示。注意对于整数 10 ，反转之后会变成 01 ，即 1 。
数组中不同整数的数目为 6（数字 1、10、12、13、21 和 31）。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,2,2]
<strong>输出:</strong> 1
<strong>解释:</strong> 反转每个数字后，结果数组是 [2,2,2,2,2,2] 。
数组中不同整数的数目为 1（数字 2）。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>6</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def countDistinctIntegers(self, nums: List[int]) -> int:
        return len(set(nums + [int(str(num)[::-1]) for num in nums]))
```
