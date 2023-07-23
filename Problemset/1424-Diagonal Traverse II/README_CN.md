# 1424. 对角线遍历 II
给你一个列表 `nums` ，里面每一个元素都是一个整数列表。请你依照下面各图的规则，按顺序返回 `nums` 中对角线上的整数。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/04/23/sample_1_1784.png)
<pre>
<strong>输入:</strong> nums = [[1,2,3],[4,5,6],[7,8,9]]
<strong>输出:</strong> [1,4,2,7,5,3,8,6,9]
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/04/23/sample_2_1784.png)
<pre>
<strong>输入:</strong> nums = [[1,2,3,4,5],[6,7],[8],[9,10,11],[12,13,14,15,16]]
<strong>输出:</strong> [1,6,2,8,7,3,9,4,12,10,5,13,11,14,15,16]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [[1,2,3],[4],[5,6,7],[8],[9,10,11]]
<strong>输出:</strong> [1,4,2,5,3,8,6,9,7,10,11]
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> nums = [[1,2,3,4,5,6]]
<strong>输出:</strong> [1,2,3,4,5,6]
</pre>

#### 提示:
* `1 <= nums.length <= 10^5`
* `1 <= nums[i].length <= 10^5`
* `1 <= nums[i][j] <= 10^9`
* `nums` 中最多有 `10^5` 个数字。

## 题解 (Ruby)

### 1. 排序
```Ruby
# @param {Integer[][]} nums
# @return {Integer[]}
def find_diagonal_order(nums)
    num_row_col = []

    for row in 0...nums.length
        for col in 0...nums[row].length
            num_row_col.push([nums[row][col], row, col])
        end
    end

    num_row_col.sort_by! {|nrc| nrc[2]}
    num_row_col.sort_by! {|nrc| nrc[1] + nrc[2]}

    return num_row_col.map {|nrc| nrc[0]}
end
```
