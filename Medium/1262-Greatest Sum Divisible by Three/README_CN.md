# 1262. 可被三整除的最大和
给你一个整数数组 ```nums```，请你找出并返回能被三整除的元素最大和。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,6,5,1,8]
<strong>输出:</strong> 18
<strong>解释:</strong> 选出数字 3, 6, 1 和 8，它们的和是 18（可被 3 整除的最大和）。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [4]
<strong>输出:</strong> 0
<strong>解释:</strong> 4 不能被 3 整除，所以无法选出数字，返回 0。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,4]
<strong>输出:</strong> 12
<strong>解释:</strong> 选出数字 1, 3, 4 以及 4，它们的和是 12（可被 3 整除的最大和）。
</pre>

#### 提示:
* ```1 <= nums.length <= 4 * 10^4```
* ```1 <= nums[i] <= 10^4```

## 题解 (Rust)

### 1. 数学
```Rust
impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut min = vec![(19996, 10000), (20000, 10001)];
        let mut ret = 0;

        for num in nums {
            ret += num;
            match num as usize % 3 {
                0 => (),
                i => {
                    if num < min[i - 1].0 {
                        min[i - 1] = (num, min[i - 1].0);
                    } else if num < min[i - 1].1 {
                        min[i - 1].1 = num;
                    }
                },
            }
        }

        match ret % 3 {
            1 => ret - min[0].0.min(min[1].0 + min[1].1),
            2 => ret - min[1].0.min(min[0].0 + min[0].1),
            _ => ret,
        }
    }
}
```
