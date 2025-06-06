# 2587. 重排数组以得到最大前缀分数
给你一个下标从 **0** 开始的整数数组 `nums` 。你可以将 `nums` 中的元素按 **任意顺序** 重排（包括给定顺序）。

令 `prefix` 为一个数组，它包含了 `nums` 重新排列后的前缀和。换句话说，`prefix[i]` 是 `nums` 重新排列后下标从 `0` 到 `i` 的元素之和。`nums` 的 **分数** 是 `prefix` 数组中正整数的个数。

返回可以得到的最大分数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,-1,0,1,-3,3,-3]
<strong>输出:</strong> 6
<strong>解释:</strong> 数组重排为 nums = [2,3,1,-1,-3,0,-3] 。
prefix = [2,5,6,5,2,2,-1] ，分数为 6 。
可以证明 6 是能够得到的最大分数。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [-2,-3,0]
<strong>输出:</strong> 0
<strong>解释:</strong> 不管怎么重排数组得到的分数都是 0 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>6</sup> <= nums[i] <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_score(mut nums: Vec<i32>) -> i32 {
        let mut prefix = vec![0; nums.len() + 1];

        nums.sort_unstable_by_key(|x| -x);

        for i in 0..nums.len() {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }

        prefix.iter().filter(|&&x| x > 0).count() as i32
    }
}
```
