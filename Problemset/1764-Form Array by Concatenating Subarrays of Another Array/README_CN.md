# 1764. 通过连接另一个数组的子数组得到一个数组
给你一个长度为 `n` 的二维整数数组 `groups` ，同时给你一个整数数组 `nums` 。

你是否可以从 `nums` 中选出 `n` 个 **不相交** 的子数组，使得第 `i` 个子数组与 `groups[i]` （下标从 **0** 开始）完全相同，且如果 `i > 0` ，那么第 `(i-1)` 个子数组在 `nums` 中出现的位置在第 `i` 个子数组前面。（也就是说，这些子数组在 `nums` 中出现的顺序需要与 `groups` 顺序相同）

如果你可以找出这样的 `n` 个子数组，请你返回 `true` ，否则返回 `false` 。

如果不存在下标为 `k` 的元素 `nums[k]` 属于不止一个子数组，就称这些子数组是 **不相交** 的。子数组指的是原数组中连续元素组成的一个序列。

#### 示例 1:
<pre>
<strong>输入:</strong> groups = [[1,-1,-1],[3,-2,0]], nums = [1,-1,0,1,-1,-1,3,-2,0]
<strong>输出:</strong> true
<strong>解释:</strong> 你可以分别在 nums 中选出第 0 个子数组 [1,-1,0,1,-1,-1,3,-2,0] 和第 1 个子数组 [1,-1,0,1,-1,-1,3,-2,0] 。
这两个子数组是不相交的，因为它们没有任何共同的元素。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> groups = [[10,-2],[1,2,3,4]], nums = [1,2,3,4,10,-2]
<strong>输出:</strong> false
<strong>解释:</strong> 选择子数组 [1,2,3,4,10,-2] 和 [1,2,3,4,10,-2] 是不正确的，因为它们出现的顺序与 groups 中顺序不同。
[10,-2] 必须出现在 [1,2,3,4] 之前。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> groups = [[1,2,3],[3,4]], nums = [7,7,1,2,3,4,7,7]
<strong>输出:</strong> false
<strong>解释:</strong> 选择子数组 [7,7,1,2,3,4,7,7] 和 [7,7,1,2,3,4,7,7] 是不正确的，因为它们不是不相交子数组。
它们有一个共同的元素 nums[4] （下标从 0 开始）。
</pre>

#### 提示:
* `groups.length == n`
* <code>1 <= n <= 10<sup>3</sup></code>
* <code>1 <= groups[i].length, sum(groups[i].length) <= 10<sup>3</sup></code>
* <code>1 <= nums.length <= 10<sup>3</sup></code>
* <code>-10<sup>7</sup> <= groups[i][j], nums[k] <= 10<sup>7</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
        let mut i = 0;

        for group in groups {
            let mut flag = true;

            loop {
                if i + group.len() > nums.len() {
                    return false;
                }

                let mut j = 0;

                while j < group.len() {
                    if nums[i + j] != group[j] {
                        flag = false;
                        break;
                    }
                    j += 1;
                }

                if flag {
                    i += j;
                    break;
                }

                i += 1;
                flag = true;
            }
        }

        true
    }
}
```
