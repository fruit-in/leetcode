# 1526. 形成目标数组的子数组最少增加次数
给你一个整数数组 `target` 和一个数组 `initial` ，`initial` 数组与 `target`  数组有同样的维度，且一开始全部为 0 。

请你返回从 `initial` 得到  `target` 的最少操作次数，每次操作需遵循以下规则：
* 在 `initial` 中选择 **任意** 子数组，并将子数组中每个元素增加 1 。

答案保证在 32 位有符号整数以内。

#### 示例 1:
<pre>
<strong>输入:</strong> target = [1,2,3,2,1]
<strong>输出:</strong> 3
<strong>解释:</strong> 我们需要至少 3 次操作从 intial 数组得到 target 数组。
[0,0,0,0,0] 将下标为 0 到 4 的元素（包含二者）加 1 。
[1,1,1,1,1] 将下标为 1 到 3 的元素（包含二者）加 1 。
[1,2,2,2,1] 将下表为 2 的元素增加 1 。
[1,2,3,2,1] 得到了目标数组。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> target = [3,1,1,2]
<strong>输出:</strong> 4
<strong>解释:</strong> (initial)[0,0,0,0] -> [1,1,1,1] -> [1,1,1,2] -> [2,1,1,2] -> [3,1,1,2] (target) 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> target = [3,1,5,4,2]
<strong>输出:</strong> 7
<strong>解释:</strong> (initial)[0,0,0,0,0] -> [1,1,1,1,1] -> [2,1,1,1,1] -> [3,1,1,1,1]
                                  -> [3,1,2,2,2] -> [3,1,3,3,2] -> [3,1,4,4,2] -> [3,1,5,4,2] (target)。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> target = [1,1,1,1]
<strong>输出:</strong> 1
</pre>

#### 提示:
* <code>1 <= target.length <= 10<sup>5</sup></code>
* <code>1 <= target[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut ret = target[0];

        for i in 1..target.len() {
            ret += 0.max(target[i] - target[i - 1]);
        }

        ret
    }
}
```
