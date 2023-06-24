# 1402. 做菜顺序
一个厨师收集了他 `n` 道菜的满意程度 `satisfaction` ，这个厨师做出每道菜的时间都是 1 单位时间。

一道菜的 「喜爱时间」系数定义为烹饪这道菜以及之前每道菜所花费的时间乘以这道菜的满意程度，也就是 `time[i]*satisfaction[i]` 。

请你返回做完所有菜 「喜爱时间」总和的最大值为多少。

你可以按 **任意** 顺序安排做菜的顺序，你也可以选择放弃做某些菜来获得更大的总和。

#### 示例 1:
<pre>
<strong>输入:</strong> satisfaction = [-1,-8,0,5,-9]
<strong>输出:</strong> 14
<strong>解释:</strong> 去掉第二道和最后一道菜，最大的喜爱时间系数和为 (-1*1 + 0*2 + 5*3 = 14) 。每道菜都需要花费 1 单位时间完成。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> satisfaction = [4,3,2]
<strong>输出:</strong> 20
<strong>解释:</strong> 去掉第二道和最后一道菜，最大的喜爱时间系数和为 (-1*1 + 0*2 + 5*3 = 14) 。每道菜都需要花费 1 单位时间完成。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> satisfaction = [-1,-4,-5]
<strong>输出:</strong> 0
<strong>解释:</strong> 大家都不喜欢这些菜，所以不做任何菜可以获得最大的喜爱时间系数。
</pre>

#### 提示:
* `n == satisfaction.length`
* `1 <= n <= 500`
* `-1000 <= satisfaction[i] <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut satisfaction = satisfaction;
        let mut sum = 0;
        let mut ret = 0;
        satisfaction.sort_unstable();

        for i in (0..satisfaction.len()).rev() {
            sum += satisfaction[i];
            if sum < 0 {
                break;
            }
            ret += sum;
        }

        ret
    }
}
```
