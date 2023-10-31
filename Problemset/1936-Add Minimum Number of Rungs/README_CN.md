# 1936. 新增的最少台阶数
给你一个 **严格递增** 的整数数组 `rungs` ，用于表示梯子上每一台阶的 **高度** 。当前你正站在高度为 `0` 的地板上，并打算爬到最后一个台阶。

另给你一个整数 `dist` 。每次移动中，你可以到达下一个距离你当前位置（地板或台阶）**不超过** `dist` 高度的台阶。当然，你也可以在任何正 **整数** 高度处插入尚不存在的新台阶。

返回爬到最后一阶时必须添加到梯子上的 **最少** 台阶数。

#### 示例 1:
<pre>
<strong>输入:</strong> rungs = [1,3,5,10], dist = 2
<strong>输出:</strong> 2
<strong>解释:</strong>
现在无法到达最后一阶。
在高度为 7 和 8 的位置增设新的台阶，以爬上梯子。
梯子在高度为 [1,3,5,7,8,10] 的位置上有台阶。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> rungs = [3,6,8,10], dist = 3
<strong>输出:</strong> 0
<strong>解释:</strong>
这个梯子无需增设新台阶也可以爬上去。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> rungs = [3,4,6,7], dist = 2
<strong>输出:</strong> 1
<strong>解释:</strong>
现在无法从地板到达梯子的第一阶。
在高度为 1 的位置增设新的台阶，以爬上梯子。
梯子在高度为 [1,3,4,6,7] 的位置上有台阶。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> rungs = [5], dist = 10
<strong>输出:</strong> 0
<strong>解释:</strong> 这个梯子无需增设新台阶也可以爬上去。
</pre>

#### 提示:
* <code>1 <= rungs.length <= 10<sup>5</sup></code>
* <code>1 <= rungs[i] <= 10<sup>9</sup></code>
* <code>1 <= dist <= 10<sup>9</sup></code>
* `rungs` **严格递增**

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn add_rungs(rungs: Vec<i32>, dist: i32) -> i32 {
        let mut height = 0;
        let mut ret = 0;

        for rung in rungs {
            if rung - height > dist {
                ret += (rung - height - 1) / dist;
            }
            height = rung;
        }

        ret
    }
}
```
