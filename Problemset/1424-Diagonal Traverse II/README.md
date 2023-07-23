# 1424. Diagonal Traverse II
Given a list of lists of integers, `nums`, return all elements of `nums` in diagonal order as shown in the below images.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/04/08/sample_1_1784.png)
<pre>
<strong>Input:</strong> nums = [[1,2,3],[4,5,6],[7,8,9]]
<strong>Output:</strong> [1,4,2,7,5,3,8,6,9]
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/04/08/sample_2_1784.png)
<pre>
<strong>Input:</strong> nums = [[1,2,3,4,5],[6,7],[8],[9,10,11],[12,13,14,15,16]]
<strong>Output:</strong> [1,6,2,8,7,3,9,4,12,10,5,13,11,14,15,16]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [[1,2,3],[4],[5,6,7],[8],[9,10,11]]
<strong>Output:</strong> [1,4,2,5,3,8,6,9,7,10,11]
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> nums = [[1,2,3,4,5,6]]
<strong>Output:</strong> [1,2,3,4,5,6]
</pre>

#### Constraints:
* `1 <= nums.length <= 10^5`
* `1 <= nums[i].length <= 10^5`
* `1 <= nums[i][j] <= 10^9`
* There at most `10^5` elements in `nums`.

## Solutions (Ruby)

### 1. Sort
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
