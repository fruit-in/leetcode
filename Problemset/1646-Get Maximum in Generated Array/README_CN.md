# 1646. 获取生成数组中的最大值
给你一个整数 `n` 。按下述规则生成一个长度为 `n + 1` 的数组 `nums` ：
* `nums[0] = 0`
* `nums[1] = 1`
* 当 `2 <= 2 * i <= n` 时，`nums[2 * i] = nums[i]`
* 当 `2 <= 2 * i + 1 <= n` 时，`nums[2 * i + 1] = nums[i] + nums[i + 1]`

返回生成数组 `nums` 中的 **最大** 值。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 7
<strong>输出:</strong> 3
<strong>解释:</strong> 根据规则：
  nums[0] = 0
  nums[1] = 1
  nums[(1 * 2) = 2] = nums[1] = 1
  nums[(1 * 2) + 1 = 3] = nums[1] + nums[2] = 1 + 1 = 2
  nums[(2 * 2) = 4] = nums[2] = 1
  nums[(2 * 2) + 1 = 5] = nums[2] + nums[3] = 1 + 2 = 3
  nums[(3 * 2) = 6] = nums[3] = 2
  nums[(3 * 2) + 1 = 7] = nums[3] + nums[4] = 2 + 1 = 3
因此，nums = [0,1,1,2,1,3,2,3]，最大值 3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 2
<strong>输出:</strong> 1
<strong>解释:</strong> 根据规则，nums[0]、nums[1] 和 nums[2] 之中的最大值是 1
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 3
<strong>输出:</strong> 2
<strong>解释:</strong> 根据规则，nums[0]、nums[1]、nums[2] 和 nums[3] 之中的最大值是 2
</pre>

#### 提示:
* `0 <= n <= 100`

## 题解 (Rust)

### 1. 动态规划
```Rust
impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        let mut nums = vec![0; n as usize + 1];

        for j in 1..nums.len() {
            nums[j] = match j / 2 {
                0 => 1,
                i if j % 2 == 0 => nums[i],
                i => nums[i] + nums[i + 1],
            };
        }

        nums.into_iter().max().unwrap()
    }
}
```
