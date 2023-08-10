# 2593. 标记所有元素后数组的分数
给你一个数组 `nums` ，它包含若干正整数。

一开始分数 `score = 0` ，请你按照下面算法求出最后分数：

* 从数组中选择最小且没有被标记的整数。如果有相等元素，选择下标最小的一个。
* 将选中的整数加到 `score` 中。
* 标记 **被选中元素**，如果有相邻元素，则同时标记 **与它相邻的两个元素** 。
* 重复此过程直到数组中所有元素都被标记。

请你返回执行上述算法后最后的分数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,1,3,4,5,2]
<strong>输出:</strong> 7
<strong>解释:</strong> 我们按照如下步骤标记元素：
- 1 是最小未标记元素，所以标记它和相邻两个元素：[2,1,3,4,5,2] 。
- 2 是最小未标记元素，所以标记它和左边相邻元素：[2,1,3,4,5,2] 。
- 4 是仅剩唯一未标记的元素，所以我们标记它：[2,1,3,4,5,2] 。
总得分为 1 + 2 + 4 = 7 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,3,5,1,3,2]
<strong>输出:</strong> 5
<strong>解释:</strong> 我们按照如下步骤标记元素：
- 1 是最小未标记元素，所以标记它和相邻两个元素：[2,3,5,1,3,2] 。
- 2 是最小未标记元素，由于有两个 2 ，我们选择最左边的一个 2 ，也就是下标为 0 处的 2 ，以及它右边相邻的元素：[2,3,5,1,3,2] 。
- 2 是仅剩唯一未标记的元素，所以我们标记它：[2,3,5,1,3,2] 。
总得分为 1 + 2 + 2 = 5 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut heap = nums
            .iter()
            .enumerate()
            .map(|(i, x)| Reverse((x, i)))
            .collect::<BinaryHeap<_>>();
        let mut indices = HashSet::new();
        let mut score = 0;

        while indices.len() < nums.len() {
            let Reverse((x, i)) = heap.pop().unwrap();

            if !indices.contains(&i) {
                score += *x as i64;
                indices.insert(i);
                if i > 0 {
                    indices.insert(i - 1);
                }
                if i < nums.len() - 1 {
                    indices.insert(i + 1);
                }
            }
        }

        score
    }
}
```
