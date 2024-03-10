# 45. 跳跃游戏 II
给定一个长度为 `n` 的 **0 索引**整数数组 `nums`。初始位置为 `nums[0]`。

每个元素 `nums[i]` 表示从索引 `i` 向前跳转的最大长度。换句话说，如果你在 `nums[i]` 处，你可以跳转到任意 `nums[i + j]` 处:

* `0 <= j <= nums[i]`
* `i + j < n`

返回到达 `nums[n - 1]` 的最小跳跃次数。生成的测试用例可以到达 `nums[n - 1]`。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,3,1,1,4]
<strong>输出:</strong> 2
<strong>解释:</strong> 跳到最后一个位置的最小跳跃数是 2。
     从下标为 0 跳到下标为 1 的位置，跳 1 步，然后跳 3 步到达数组的最后一个位置。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,3,0,1,4]
<strong>输出:</strong> 2
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>4</sup></code>
* `0 <= nums[i] <= 1000`
* 题目保证可以到达 `nums[n-1]`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let mut ret = 0;

        for k in 0..nums.len() {
            if k > i {
                i = j;
                ret += 1;
            }
            j = j.max(k + nums[k] as usize);
        }

        ret
    }
}
```
