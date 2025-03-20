# 2610. 转换二维数组
给你一个整数数组 `nums` 。请你创建一个满足以下条件的二维数组：
* 二维数组应该 **只** 包含数组 `nums` 中的元素。
* 二维数组中的每一行都包含 **不同** 的整数。
* 二维数组的行数应尽可能 **少** 。

返回结果数组。如果存在多种答案，则返回其中任何一种。

请注意，二维数组的每一行上可以存在不同数量的元素。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,3,4,1,2,3,1]
<strong>输出:</strong> [[1,3,4,2],[1,3],[1]]
<strong>解释:</strong> 根据题目要求可以创建包含以下几行元素的二维数组：
- 1,3,4,2
- 1,3
- 1
nums 中的所有元素都有用到，并且每一行都由不同的整数组成，所以这是一个符合题目要求的答案。
可以证明无法创建少于三行且符合题目要求的二维数组。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,4]
<strong>输出:</strong> [[4,3,2,1]]
<strong>解释:</strong> nums 中的所有元素都不同，所以我们可以将其全部保存在二维数组中的第一行。
</pre>

#### 提示:
* `1 <= nums.length <= 200`
* `1 <= nums[i] <= nums.length`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_matrix(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut prev = 0;
        let mut count = 0;
        let mut ret = vec![];

        nums.sort_unstable();

        for &x in &nums {
            if x != prev {
                prev = x;
                count = 0;
            }
            count += 1;
            if count > ret.len() {
                ret.push(vec![]);
            }
            ret[count - 1].push(x);
        }

        ret
    }
}
```
