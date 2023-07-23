# 1304. 和为零的N个唯一整数
给你一个整数 ```n```，请你返回 **任意** 一个由 ```n``` 个 **各不相同** 的整数组成的数组，并且这 ```n``` 个数相加和为 ```0``` 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 5
<strong>输出:</strong> [-7,-1,1,3,4]
<strong>解释:</strong> 这些数组也是正确的 [-5,-1,1,2,3]，[-3,-1,2,-2,4]。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 3
<strong>输出:</strong> [-1,0,1]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> [0]
</pre>

#### 提示:
* ```1 <= n <= 1000```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        ((1 - n)..=n).step_by(2).collect()
    }
}
```
