# 665. 非递减数列
给定一个长度为 ```n``` 的整数数组，你的任务是判断在**最多**改变 ```1``` 个元素的情况下，该数组能否变成一个非递减数列。

我们是这样定义一个非递减数列的： 对于数组中所有的 ```i``` (1 <= i < n)，满足 ```array[i] <= array[i + 1]```。

#### 示例 1:
<pre>
<strong>输入:</strong> [4,2,3]
<strong>输出:</strong> True
<strong>解释:</strong> 你可以通过把第一个4变成1来使得它成为一个非递减数列。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [4,2,1]
<strong>输出:</strong> False
<strong>解释:</strong> 你不能在只改变一个元素的情况下将其变为非递减数列。
</pre>

**说明:** ```n``` 的范围为 [1, 10,000]。

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def checkPossibility(self, nums: List[int]) -> bool:
        flag = False
        for i in range(len(nums) - 1):
            if nums[i] > nums[i + 1]:
                if flag:
                    return False
                flag = True
                if i > 0 and nums[i - 1] > nums[i + 1]:
                    nums[i + 1] = nums[i]
        return True
```
