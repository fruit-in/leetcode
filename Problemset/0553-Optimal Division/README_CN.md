# 553. 最优除法
给定一正整数数组 `nums`，`nums` 中的相邻整数将进行浮点除法。例如， [2,3,4] -> 2 / 3 / 4 。

* 例如，`nums = [2,3,4]`，我们将求表达式的值 `"2/3/4"`。

但是，你可以在任意位置添加任意数目的括号，来改变算数的优先级。你需要找出怎么添加括号，以便计算后的表达式的值为最大值。

以字符串格式返回具有最大值的对应表达式。

**注意：**你的表达式不应该包含多余的括号。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1000,100,10,2]
<strong>输出:</strong> "1000/(100/10/2)"
<strong>解释:</strong> 1000/(100/10/2) = 1000/((100/10)/2) = 200
但是，以下加粗的括号 "1000/((100/10)/2)" 是冗余的，
因为他们并不影响操作的优先级，所以你需要返回 "1000/(100/10/2)"。

其他用例:
1000/(100/10)/2 = 50
1000/(100/(10/2)) = 50
1000/100/10/2 = 0.5
1000/100/(10/2) = 2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,3,4]
<strong>输出:</strong> "2/(3/4)"
<strong>解释:</strong> (2/(3/4)) = 8/3 = 2.667
可以看出，在尝试了所有的可能性之后，我们无法得到一个结果大于 2.667 的表达式。
</pre>

#### 说明:
* `1 <= nums.length <= 10`
* `2 <= nums[i] <= 1000`
* 对于给定的输入只有一种最优除法。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def optimalDivision(self, nums: List[int]) -> str:
        expression = "/".join(map(str, nums[1:]))
        if len(nums) > 2:
            expression = "(" + expression + ")"
        if len(nums) > 1:
            expression = "/" + expression
        expression = str(nums[0]) + expression

        return expression
```
