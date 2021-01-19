# 611. 有效三角形的个数
给定一个包含非负整数的数组，你的任务是统计其中可以组成三角形三条边的三元组个数。

#### 示例 1:
<pre>
<strong>输入:</strong> [2,2,3,4]
<strong>输出:</strong> 3
<strong>解释:</strong>
有效的组合是:
2,3,4 (使用第一个 2)
2,3,4 (使用第二个 2)
2,2,3
</pre>

#### 注意:
1. 数组长度不超过1000。
2. 数组里整数的范围为 [0, 1000]。

## 题解 (Rust)

### 1. 二分查找
```Rust
impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        let mut ret = 0;

        nums.sort_unstable();

        for i in 0..nums.len() {
            if nums[i] > 0 {
                for j in (i + 1)..nums.len() {
                    match nums[(j + 1)..].binary_search(&(nums[i] + nums[j] - 1)) {
                        Ok(k) => ret += k + 1,
                        Err(k) => ret += k,
                    }
                }
            }
        }

        ret as i32
    }
}
```
