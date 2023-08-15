# 553. Optimal Division
You are given an integer array `nums`. The adjacent integers in `nums` will perform the float division.

* For example, for `nums = [2,3,4]`, we will evaluate the expression `"2/3/4"`.

However, you can add any number of parenthesis at any position to change the priority of operations. You want to add these parentheses such the value of the expression after the evaluation is maximum.

Return *the corresponding expression that has the maximum value in string format*.

**Note:** your expression should not contain redundant parenthesis.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1000,100,10,2]
<strong>Output:</strong> "1000/(100/10/2)"
<strong>Explanation:</strong> 1000/(100/10/2) = 1000/((100/10)/2) = 200
However, the bold parenthesis in "1000/((100/10)/2)" are redundant since they do not influence the operation priority.
So you should return "1000/(100/10/2)".
Other cases:
1000/(100/10)/2 = 50
1000/(100/(10/2)) = 50
1000/100/10/2 = 0.5
1000/100/(10/2) = 2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,3,4]
<strong>Output:</strong> "2/(3/4)"
<strong>Explanation:</strong> (2/(3/4)) = 8/3 = 2.667
It can be shown that after trying all possibilities, we cannot get an expression with evaluation greater than 2.667
</pre>

#### Constraints:
* `1 <= nums.length <= 10`
* `2 <= nums[i] <= 1000`
* There is only one optimal division for the given input.

## Solutions (Python)

### 1. Solution
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
