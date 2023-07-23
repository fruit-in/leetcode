# 179. 最大数
给定一组非负整数，重新排列它们的顺序使之组成一个最大的整数。

#### 示例 1:
<pre>
<strong>输入:</strong> [10,2]
<strong>输出:</strong> 210
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [3,30,34,5,9]
<strong>输出:</strong> 9534330
</pre>

**说明:** 输出结果可能非常大，所以你需要返回一个字符串而不是整数。

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.iter().map(|num| num.to_string()).collect::<Vec<_>>();
        nums.sort_unstable_by(|a, b| (b.to_owned() + a).cmp(&(a.to_owned() + b)));

        if nums.len() > 0 && nums[0] == 0.to_string() {
            return 0.to_string();
        }
        nums.concat()
    }
}
```
